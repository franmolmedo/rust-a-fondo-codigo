#![deny(unsafe_op_in_unsafe_fn)]

/// # Safety
/// `pointer` must be aligned, readable, and point to a valid initialized T.
/// No incompatible access, including a concurrent write, may occur during
/// this read. Copying the value must respect the validity rules of T.
unsafe fn read_copy<T: Copy>(pointer: *const T) -> T {
    // SAFETY: The caller guarantees that `pointer` is aligned, readable,
    // and points to an initialized `T` for the duration of this read.
    unsafe { pointer.read() }
}

let value = 21_u32;
// SAFETY: `&value` produces a valid, aligned pointer to initialized data.
assert_eq!(unsafe { read_copy(&value) }, 21);
