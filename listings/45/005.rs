use std::mem::align_of;
use std::ptr::NonNull;

let pointer = NonNull::<u64>::dangling();
assert_ne!(pointer.as_ptr().addr(), 0);
assert_eq!(pointer.as_ptr().addr() % align_of::<u64>(), 0);

// Do not dereference it: being non-null and aligned does not make it accessible.
