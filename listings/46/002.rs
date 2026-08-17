use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Header {
    kind: u8,
    payload_length: u32,
    version: u16,
}

assert_eq!(offset_of!(Header, kind), 0);
assert!(offset_of!(Header, kind) < offset_of!(Header, payload_length));
assert!(offset_of!(Header, payload_length) < offset_of!(Header, version));
assert_eq!(size_of::<Header>() % align_of::<Header>(), 0);
