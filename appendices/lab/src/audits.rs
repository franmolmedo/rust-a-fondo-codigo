//! A deliberately small unsafe boundary for the Miri lab.

/// Read the first four bytes as a little-endian integer.
pub fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    let prefix = bytes.get(..4)?;
    // SAFETY: prefix refers to four initialized bytes in one live allocation.
    // u32 accepts every bit pattern. read_unaligned has no alignment requirement,
    // does not write through the shared borrow, and does not retain the pointer.
    let native = unsafe { prefix.as_ptr().cast::<u32>().read_unaligned() };
    Some(u32::from_le(native))
}

/// Prefer this safe version unless measurements justify an unsafe alternative.
pub fn read_u32_le_safe(bytes: &[u8]) -> Option<u32> {
    let array: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(array))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_inputs_are_rejected() {
        for length in 0..4 {
            assert_eq!(read_u32_le(&[0; 4][..length]), None);
        }
    }

    #[test]
    fn offsets_and_trailing_bytes_are_supported() {
        let bytes = [0, 0x78, 0x56, 0x34, 0x12, 99];
        assert_eq!(read_u32_le(&bytes[1..]), Some(0x1234_5678));
    }

    #[test]
    fn safe_and_unsafe_versions_agree() {
        for value in [0_u32, 1, 255, 65_536, 0x1234_5678, u32::MAX] {
            let bytes = value.to_le_bytes();
            assert_eq!(read_u32_le(&bytes), Some(value));
            assert_eq!(read_u32_le(&bytes), read_u32_le_safe(&bytes));
        }
    }
}
