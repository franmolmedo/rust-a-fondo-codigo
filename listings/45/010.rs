#[repr(align(2))]
struct Word(u16);

let value = Word(42);
let base = &value as *const Word;
let tagged = base.map_addr(|address| address | 1);

assert_eq!(tagged.addr() & 1, 1);
let restored = tagged.map_addr(|address| address & !1);

// SAFETY: this restores the exact address of `value`; `map_addr` kept its
// provenance, and `value` remains borrowed for shared access.
assert_eq!(unsafe { (*restored).0 }, 42);
