/// Reads one copyable value from a raw pointer.
///
/// # Safety
///
/// `pointer` must be non-null, properly aligned, initialized for `T` and
/// valid for a read during this call. No conflicting access may occur.
unsafe fn read_one<T: Copy>(pointer: *const T) -> T {
    // SAFETY: the caller provides exactly the preconditions required by `read`.
    unsafe { pointer.read() }
}

fn main() {
    let value = 42;
    // SAFETY: the pointer comes from this live, aligned, initialized `i32`.
    assert_eq!(unsafe { read_one(&raw const value) }, 42);
}
