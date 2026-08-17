const MAGIC: [u8; 2] = *b"DR";

fn encode_header(version: u16, tag: u8, length: u32) -> [u8; 9] {
    let mut bytes = [0; 9];
    bytes[0..2].copy_from_slice(&MAGIC);
    bytes[2..4].copy_from_slice(&version.to_le_bytes());
    bytes[4] = tag;
    bytes[5..9].copy_from_slice(&length.to_le_bytes());
    bytes
}

assert_eq!(
    encode_header(3, 2, 65_537),
    [b'D', b'R', 3, 0, 2, 1, 0, 1, 0],
);
