use std::pin::Pin;

let mut number = 42_u64;
let pinned: Pin<&mut u64> = Pin::new(&mut number);
let normal: &mut u64 = pinned.get_mut(); // u64: Unpin, sin restricción real
*normal += 1;
assert_eq!(number, 43);
