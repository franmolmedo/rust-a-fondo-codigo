use std::convert::TryFrom;

let big: u64 = 300;
let small = u8::try_from(big);
assert!(small.is_err());
