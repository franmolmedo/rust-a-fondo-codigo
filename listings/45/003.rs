/// # Safety
/// `pointer` must be non-null, aligned and point to a valid `T` for all of `'a`.
/// The caller must uphold shared-reference aliasing and lifetime requirements.
unsafe fn forged_ref<'a, T>(pointer: *const T) -> &'a T {
    // SAFETY: this is correct only if the caller contract proves every
    // reference requirement, including the chosen `'a`.
    unsafe { &*pointer }
}
