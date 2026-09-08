# Portable appendix examples

## AP-G-B001

```rust
use portable_core::Ring;

let mut queue = Ring::<u8, 3>::new();
assert_eq!(queue.push(10), Ok(()));
assert_eq!(queue.push(20), Ok(()));
assert_eq!(queue.pop(), Some(10));
assert_eq!(queue.front(), Some(20));
assert_eq!(queue.len(), 1);
```

## AP-G-B002

```rust
use portable_core::Ring;

let mut queue = Ring::<u8, 2>::new();
queue.push(1).unwrap();
queue.push(2).unwrap();
assert_eq!(queue.push(3), Err(3));
assert_eq!(queue.pop(), Some(1));
queue.push(3).unwrap();
assert_eq!([queue.pop(), queue.pop()], [Some(2), Some(3)]);
```

## AP-G-B003

```rust,compile_fail
use portable_core::Ring;

fn main() {
    let _ = Ring::<String, 4>::new();
}
```

## AP-G-B004

```rust
use portable_core::{ByteSink, Ring, flush_to};

struct OneByte(Option<u8>);

impl ByteSink for OneByte {
    type Error = &'static str;

    fn send(&mut self, byte: u8) -> Result<(), Self::Error> {
        if self.0.is_some() {
            return Err("full");
        }
        self.0 = Some(byte);
        Ok(())
    }
}

let mut queue = Ring::<u8, 2>::new();
queue.push(7).unwrap();
queue.push(8).unwrap();
let mut sink = OneByte(None);
let error = flush_to(&mut queue, &mut sink).unwrap_err();
assert_eq!(error.written, 1);
assert_eq!(sink.0, Some(7));
assert_eq!(queue.front(), Some(8));
```

## AP-G-B006

```rust
#[cfg(feature = "alloc")]
fn main() {
    use portable_core::Ring;

    let mut queue = Ring::<u8, 2>::new();
    queue.push(7).unwrap();
    queue.push(8).unwrap();
    assert_eq!(queue.try_drain_to_vec().unwrap(), [7, 8]);
    assert!(queue.is_empty());
}

#[cfg(not(feature = "alloc"))]
fn main() {}
```
