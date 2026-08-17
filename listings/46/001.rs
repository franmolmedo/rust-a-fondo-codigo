use std::mem::{align_of, size_of};

assert_eq!(size_of::<u32>(), 4);
assert!(align_of::<u32>().is_power_of_two());
assert_eq!(size_of::<u32>() % align_of::<u32>(), 0);
