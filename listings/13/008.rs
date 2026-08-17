use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingPort,
    InvalidPort,
}

fn required_port(value: Option<&str>) -> Result<u16, ConfigError> {
    value
        .ok_or(ConfigError::MissingPort)?
        .parse::<u16>()
        .map_err(|_| ConfigError::InvalidPort)
}

fn optional_port(value: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    value.map(str::parse::<u16>).transpose()
}

fn main() {
    assert_eq!(required_port(Some("8080")), Ok(8080));
    assert_eq!(required_port(None), Err(ConfigError::MissingPort));
    assert_eq!(optional_port(None), Ok(None));
    assert_eq!(optional_port(Some("8080")), Ok(Some(8080)));
}
