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

// SAFETY: el campo está inicializado dentro de `packet`; el raw borrow evita
// crear `&u32` y `read_unaligned` admite la alineación reducida.
let length = unsafe { pointer.read_unaligned() };
assert_eq!(length, 42);
