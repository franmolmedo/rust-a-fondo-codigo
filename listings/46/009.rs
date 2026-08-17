#[repr(align(64))]
struct CacheLine<T>(T);

assert_eq!(std::mem::align_of::<CacheLine<u8>>(), 64);
assert_eq!(std::mem::size_of::<CacheLine<u8>>(), 64);

let lines = [CacheLine(10_u8), CacheLine(20_u8)];
let distance = (&raw const lines[1]).addr() - (&raw const lines[0]).addr();
assert_eq!(distance, 64);
