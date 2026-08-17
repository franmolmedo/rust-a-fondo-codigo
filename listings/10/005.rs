#[derive(Debug, PartialEq)]
enum PortError {
    Empty,
    NotANumber,
    Reserved { port: u16 },
}

fn parse_port(input: &str) -> Result<u16, PortError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(PortError::Empty);
    }

    let port = input.parse::<u16>().map_err(|_| PortError::NotANumber)?;
    if port < 1024 {
        return Err(PortError::Reserved { port });
    }

    Ok(port)
}

fn main() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port("80"), Err(PortError::Reserved { port: 80 }));
}
