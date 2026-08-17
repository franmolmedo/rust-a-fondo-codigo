use std::mem::{align_of, size_of};

assert_eq!(size_of::<()>(), 0);
assert_eq!(align_of::<()>(), 1);
assert_eq!(size_of::<[(); 1_000]>(), 0);
