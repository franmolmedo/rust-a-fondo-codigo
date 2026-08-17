#[derive(Debug, PartialEq, Eq)]
struct Port(u16);

#[derive(Debug, PartialEq, Eq)]
enum PortError {
    Zero,
    InvalidNumber,
}

impl TryFrom<&str> for Port {
    type Error = PortError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let value = raw.parse::<u16>().map_err(|_| PortError::InvalidNumber)?;
        (value != 0).then_some(Self(value)).ok_or(PortError::Zero)
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> Self {
        port.0
    }
}

fn main() {
    let port = Port::try_from("8080").unwrap();
    assert_eq!(u16::from(port), 8080);
    assert_eq!(Port::try_from("0"), Err(PortError::Zero));
}
