use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct ConfigError {
    field: &'static str,
    source: ParseIntError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "configuración inválida en {}", self.field)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn parse_workers(input: &str) -> Result<u16, ConfigError> {
    input.parse::<u16>().map_err(|source| ConfigError {
        field: "workers",
        source,
    })
}

fn main() {
    let error = parse_workers("many").unwrap_err();
    assert_eq!(error.to_string(), "configuración inválida en workers");
    assert!(error.source().is_some());
}
