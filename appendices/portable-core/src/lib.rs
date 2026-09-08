#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../doctests.md")]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(test)]
extern crate std;

/// A fixed-capacity FIFO for Copy values. Capacity zero is valid but always full.
pub struct Ring<T: Copy, const N: usize> {
    slots: [Option<T>; N],
    head: usize,
    len: usize,
}

impl<T: Copy, const N: usize> Ring<T, N> {
    pub const fn new() -> Self {
        Self {
            slots: [None; N],
            head: 0,
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }
    pub const fn capacity(&self) -> usize {
        N
    }
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        // Wrap without computing head+len when that sum could overflow usize.
        let tail = if self.len >= N - self.head {
            self.len - (N - self.head)
        } else {
            self.head + self.len
        };
        self.slots[tail] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn front(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.slots[self.head]
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = self.slots[self.head].take();
        self.head = if self.head == N - 1 { 0 } else { self.head + 1 };
        self.len -= 1;
        value
    }

    #[cfg(feature = "alloc")]
    pub fn try_drain_to_vec(
        &mut self,
    ) -> Result<alloc::vec::Vec<T>, alloc::collections::TryReserveError> {
        let mut values = alloc::vec::Vec::new();
        // Reserve first: an allocation error must leave the queue unchanged.
        values.try_reserve(self.len)?;
        while let Some(value) = self.pop() {
            values.push(value);
        }
        Ok(values)
    }
}

impl<T: Copy, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// An adapter must return Err only if it did not accept the byte. This is
/// a domain contract, not something the compiler can prove for an implementor.
pub trait ByteSink {
    type Error;
    fn send(&mut self, byte: u8) -> Result<(), Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlushError<E> {
    pub written: usize,
    pub source: E,
}

/// Remove each byte only after the adapter accepts it. An error reports progress.
pub fn flush_to<S: ByteSink, const N: usize>(
    queue: &mut Ring<u8, N>,
    sink: &mut S,
) -> Result<usize, FlushError<S::Error>> {
    let mut written = 0;
    while let Some(byte) = queue.front() {
        sink.send(byte)
            .map_err(|source| FlushError { written, source })?;
        let _ = queue.pop();
        written += 1;
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn zero_capacity_never_indexes_storage() {
        let mut queue = Ring::<u8, 0>::new();
        assert!(queue.is_empty());
        assert!(queue.is_full());
        assert_eq!(queue.push(7), Err(7));
        assert_eq!(queue.front(), None);
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn single_slot_can_be_reused() {
        let mut queue = Ring::<u8, 1>::default();
        for byte in 0..=255 {
            assert_eq!(queue.push(byte), Ok(()));
            assert_eq!(queue.push(9), Err(9));
            assert_eq!(queue.pop(), Some(byte));
        }
    }

    #[test]
    fn wraparound_preserves_fifo_order() {
        let mut queue = Ring::<u8, 3>::new();
        for byte in [1, 2, 3] {
            queue.push(byte).unwrap();
        }
        assert_eq!(queue.pop(), Some(1));
        queue.push(4).unwrap();
        assert_eq!(
            [queue.pop(), queue.pop(), queue.pop()],
            [Some(2), Some(3), Some(4)]
        );
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn all_short_operation_sequences_match_vec_deque() {
        for mask in 0_u32..1024 {
            let mut queue = Ring::<u8, 3>::new();
            let mut reference = VecDeque::new();
            for step in 0..10 {
                if mask & (1 << step) == 0 {
                    let expected = if reference.len() == 3 {
                        Err(step)
                    } else {
                        reference.push_back(step);
                        Ok(())
                    };
                    assert_eq!(queue.push(step), expected);
                } else {
                    assert_eq!(queue.pop(), reference.pop_front());
                }
                assert_eq!(queue.len(), reference.len());
                assert_eq!(queue.front(), reference.front().copied());
            }
        }
    }

    struct LimitedSink {
        bytes: Vec<u8>,
        remaining: usize,
    }
    impl ByteSink for LimitedSink {
        type Error = &'static str;
        fn send(&mut self, byte: u8) -> Result<(), Self::Error> {
            if self.remaining == 0 {
                return Err("full");
            }
            self.bytes.push(byte);
            self.remaining -= 1;
            Ok(())
        }
    }

    #[test]
    fn adapter_error_preserves_unsent_bytes_and_reports_progress() {
        let mut queue = Ring::<u8, 3>::new();
        for byte in [1, 2, 3] {
            queue.push(byte).unwrap();
        }
        let mut sink = LimitedSink {
            bytes: Vec::new(),
            remaining: 2,
        };
        assert_eq!(
            flush_to(&mut queue, &mut sink),
            Err(FlushError {
                written: 2,
                source: "full"
            })
        );
        assert_eq!(sink.bytes, vec![1, 2]);
        assert_eq!(queue.front(), Some(3));
        sink.remaining = 1;
        assert_eq!(flush_to(&mut queue, &mut sink), Ok(1));
        assert_eq!(sink.bytes, vec![1, 2, 3]);
        assert!(queue.is_empty());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn optional_allocation_preserves_queue_order() {
        let mut queue = Ring::<u8, 2>::new();
        queue.push(7).unwrap();
        queue.push(8).unwrap();
        assert_eq!(queue.try_drain_to_vec().unwrap(), vec![7, 8]);
        assert!(queue.is_empty());
    }
}
