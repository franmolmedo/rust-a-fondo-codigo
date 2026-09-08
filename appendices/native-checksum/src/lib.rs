//! A real, statically linked C function behind a safe Rust interface.

unsafe extern "C" {
    fn appendix_byte_sum(bytes: *const u8, length: usize) -> u32;
}

/// Sum bytes modulo 2^32. This is not cryptographic authentication.
pub fn byte_sum(bytes: &[u8]) -> u32 {
    // SAFETY: the slice supplies length initialized bytes for the whole call.
    // C neither modifies the allocation nor retains the pointer. It does not
    // dereference the pointer for an empty slice. u8/u32 and usize correspond
    // to the C uint8_t/uint32_t and size_t on the Windows/Linux targets covered
    // by this lab, with the C ABI selected explicitly above.
    unsafe { appendix_byte_sum(bytes.as_ptr(), bytes.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_handles_empty_and_nonempty_inputs() {
        assert_eq!(byte_sum(&[]), 0);
        assert_eq!(byte_sum(b"Rust"), 430);
        assert_eq!(byte_sum("á".as_bytes()), 356);
        assert_eq!(byte_sum(&[255; 10]), 2550);
    }

    #[test]
    fn c_matches_a_rust_reference() {
        let bytes: Vec<_> = (0..=255).cycle().take(10_000).collect();
        let expected = bytes
            .iter()
            .fold(0_u32, |sum, &byte| sum.wrapping_add(u32::from(byte)));
        assert_eq!(byte_sum(&bytes), expected);
    }
}
