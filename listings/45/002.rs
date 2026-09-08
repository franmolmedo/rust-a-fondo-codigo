#[repr(C, packed)]
struct Header {
    kind: u8,
    sequence: u32,
}

let header = Header {
    kind: 3,
    sequence: 0x1020_3040,
};
let pointer = &raw const header.sequence;

// SAFETY: the pointer targets the initialized field; `read_unaligned` does
// not require the natural `u32` alignment that a reference would require.
let sequence = unsafe { pointer.read_unaligned() };
assert_eq!(sequence, 0x1020_3040);
