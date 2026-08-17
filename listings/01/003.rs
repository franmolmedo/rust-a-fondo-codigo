fn first_byte(bytes: &[u8]) -> Option<u8> {
    if bytes.is_empty() {
        None
    } else {
        // SAFETY: acabamos de comprobar que el índice 0 existe.
        Some(unsafe { *bytes.get_unchecked(0) })
    }
}

fn main() {
    assert_eq!(first_byte(b"rust"), Some(b'r'));
    assert_eq!(first_byte(b""), None);
}
