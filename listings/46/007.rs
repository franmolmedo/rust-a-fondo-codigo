use std::mem::{align_of, size_of};
use std::ptr::NonNull;

assert_eq!(
    size_of::<Option<NonNull<u32>>>(),
    size_of::<NonNull<u32>>(),
);
assert_eq!(
    align_of::<Option<NonNull<u32>>>(),
    align_of::<NonNull<u32>>(),
);
