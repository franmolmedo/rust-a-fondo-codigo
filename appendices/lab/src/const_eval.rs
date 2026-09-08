//! Compile-time and runtime validation share the same implementation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl Port {
    /// This application's configuration requires a nonzero port.
    /// Operating systems can also accept zero when choosing an ephemeral port.
    pub const fn new(value: u16) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

pub const DEFAULT_PORT: Port = match Port::new(8042) {
    Some(port) => port,
    None => panic!("the default port must be nonzero"),
};

/// A byte sum modulo 2^32, not a cryptographic integrity check.
pub const fn byte_sum(bytes: &[u8]) -> u32 {
    let mut sum = 0_u32;
    let mut index = 0;
    while index < bytes.len() {
        sum = sum.wrapping_add(bytes[index] as u32);
        index += 1;
    }
    sum
}

pub const fn squares() -> [u16; 16] {
    let mut table = [0; 16];
    let mut index = 0;
    while index < table.len() {
        // The loop bounds keep index <= 15, so neither operation can overflow.
        table[index] = (index as u16) * (index as u16);
        index += 1;
    }
    table
}

pub const SQUARES: [u16; 16] = squares();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> Frame<N> {
    /// This example accepts frames of 1 through 64 bytes.
    pub const fn new(bytes: [u8; N]) -> Option<Self> {
        if N == 0 || N > 64 {
            None
        } else {
            Some(Self { bytes })
        }
    }

    pub const fn bytes(&self) -> &[u8; N] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_validate_at_both_call_sites() {
        const PORT: Option<Port> = Port::new(1234);
        assert_eq!(PORT.map(Port::get), Some(1234));
        let runtime_value = "1234".parse().unwrap();
        assert_eq!(Port::new(runtime_value), PORT);
        assert_eq!(Port::new(0), None);
        assert_eq!(Port::new(u16::MAX).unwrap().get(), u16::MAX);
        assert_eq!(DEFAULT_PORT.get(), 8042);
    }

    #[test]
    fn byte_sums_include_every_byte() {
        const SUM: u32 = byte_sum(b"Rust");
        assert_eq!(SUM, 430);
        assert_eq!(byte_sum(&[]), 0);
        assert_eq!(byte_sum("á".as_bytes()), 356);
    }

    #[test]
    fn table_covers_both_ends() {
        for (index, value) in SQUARES.iter().enumerate() {
            assert_eq!(usize::from(*value), index * index);
        }
    }

    #[test]
    fn frames_check_all_boundaries() {
        assert!(Frame::new([]).is_none());
        assert_eq!(Frame::new([7]).unwrap().bytes(), &[7]);
        assert!(Frame::new([0; 64]).is_some());
        assert!(Frame::new([0; 65]).is_none());
    }
}
