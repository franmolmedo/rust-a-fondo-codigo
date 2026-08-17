let bytes = [0x78, 0x56, 0x34, 0x12];
let value = u32::from_le_bytes(bytes);
assert_eq!(value, 0x1234_5678);
assert_eq!("Rust".as_bytes(), b"Rust");
