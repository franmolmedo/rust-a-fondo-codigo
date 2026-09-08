#[derive(Debug, PartialEq)]
enum PortError {
    Empty,
    InvalidNumber,
    Reserved { port: u16 },
}

fn parse_port(input: &str) -> Result<u16, PortError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(PortError::Empty);
    }

    let port = input.parse::<u16>().map_err(|_| PortError::InvalidNumber)?;
    if port < 1024 {
        return Err(PortError::Reserved { port });
    }

    Ok(port)
}

fn main() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port("80"), Err(PortError::Reserved { port: 80 }));
    assert_eq!(parse_port(" "), Err(PortError::Empty));
    assert_eq!(parse_port("abc"), Err(PortError::InvalidNumber));
    assert_eq!(parse_port("65536"), Err(PortError::InvalidNumber));
    assert_eq!(parse_port("1024"), Ok(1024));
}
