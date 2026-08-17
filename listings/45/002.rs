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

// SAFETY: el puntero señala el campo inicializado; `read_unaligned` no exige
// la alineación natural de `u32` que una referencia sí exigiría.
let sequence = unsafe { pointer.read_unaligned() };
assert_eq!(sequence, 0x1020_3040);
