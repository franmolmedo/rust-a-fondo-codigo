#![deny(unsafe_op_in_unsafe_fn)]

/// Advances a pointer inside its allocation.
///
/// # Safety
///
/// `pointer.add(count)` must remain in-bounds or one-past the same allocation.
unsafe fn advance<T>(pointer: *const T, count: usize) -> *const T {
    // SAFETY: guaranteed by the caller contract above.
    unsafe { pointer.add(count) }
}

fn main() {}
