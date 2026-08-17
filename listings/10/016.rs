use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
enum UsernameError {
    Empty,
    TooShort { minimum: usize, actual: usize },
}

impl fmt::Display for UsernameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "el nombre no puede estar vacío"),
            Self::TooShort { minimum, actual } => write!(
                formatter,
                "el nombre necesita al menos {minimum} caracteres y tiene {actual}"
            ),
        }
    }
}

impl Error for UsernameError {}

fn main() {
    let error = UsernameError::TooShort {
        minimum: 3,
        actual: 1,
    };
    assert_eq!(error.to_string(), "el nombre necesita al menos 3 caracteres y tiene 1");
}
