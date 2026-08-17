use std::error::Error;
use std::fmt;

trait CodedError: Error {
    fn code(&self) -> &'static str;
}

#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
}

impl fmt::Display for EmailError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "email vacío"),
            Self::MissingAt => write!(formatter, "falta @"),
        }
    }
}

impl Error for EmailError {}

impl CodedError for EmailError {
    fn code(&self) -> &'static str {
        match self {
            Self::Empty => "email.empty",
            Self::MissingAt => "email.missing_at",
        }
    }
}

fn public_code(error: &dyn CodedError) -> &'static str {
    error.code()
}

fn main() {
    let error = EmailError::MissingAt;
    assert_eq!(public_code(&error), "email.missing_at");
    assert_eq!(error, EmailError::MissingAt);
}
