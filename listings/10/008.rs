use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    InvalidNumber(ParseIntError),
    ZeroPort,
}

impl From<ParseIntError> for ConfigError {
    fn from(source: ParseIntError) -> Self {
        Self::InvalidNumber(source)
    }
}

fn parse_port(input: &str) -> Result<u16, ConfigError> {
    let port = input.parse::<u16>()?;
    if port == 0 {
        return Err(ConfigError::ZeroPort);
    }
    Ok(port)
}

fn main() {
    assert_eq!(parse_port("8080").unwrap(), 8080);
    assert!(matches!(parse_port("abc"), Err(ConfigError::InvalidNumber(_))));
}
