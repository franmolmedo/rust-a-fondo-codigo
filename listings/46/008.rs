#[repr(C, packed)]
struct Packet {
    tag: u8,
    payload_length: u32,
}

let packet = Packet {
    tag: 1,
    payload_length: 42,
};
let pointer = &raw const packet.payload_length;

// SAFETY: the field is initialized inside `packet`; the raw borrow avoids
// creating `&u32`, and `read_unaligned` accepts the reduced alignment.
let length = unsafe { pointer.read_unaligned() };
assert_eq!(length, 42);
