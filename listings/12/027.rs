use std::iter::FusedIterator;

#[derive(Debug)]
struct Countdown {
    next: u32,
}

impl Countdown {
    fn new(start: u32) -> Self {
        Self { next: start }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            return None;
        }
        let current = self.next;
        self.next -= 1;
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.next as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Countdown {}
impl FusedIterator for Countdown {}

fn main() {
    let mut countdown = Countdown::new(3);
    assert_eq!(countdown.len(), 3);
    assert_eq!(countdown.by_ref().collect::<Vec<_>>(), [3, 2, 1]);
    assert_eq!(countdown.next(), None);
}
