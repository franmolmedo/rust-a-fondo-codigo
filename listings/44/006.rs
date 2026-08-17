/// Exposes one contiguous region readable for the lifetime of `&self`.
///
/// # Safety
///
/// The pointer-length pair must be non-null, initialized, contained in one
/// live allocation and free from conflicting mutation while `self` is borrowed.
unsafe trait ContiguousBytes {
    fn raw_parts(&self) -> (*const u8, usize);
}

// SAFETY: an array owns exactly N initialized contiguous bytes.
unsafe impl<const N: usize> ContiguousBytes for [u8; N] {
    fn raw_parts(&self) -> (*const u8, usize) {
        (self.as_ptr(), self.len())
    }
}
