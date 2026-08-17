use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ErrorCategory {
    Validation,
    Conflict,
    Infrastructure,
}

trait DomainError: Error + Send + Sync + 'static {
    fn code(&self) -> &'static str;
    fn category(&self) -> ErrorCategory;
}

#[derive(Debug)]
struct EmptyEmail;

impl fmt::Display for EmptyEmail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "el email no puede estar vacío")
    }
}

impl Error for EmptyEmail {}

impl DomainError for EmptyEmail {
    fn code(&self) -> &'static str {
        "email.empty"
    }

    fn category(&self) -> ErrorCategory {
        ErrorCategory::Validation
    }
}

fn validate_email(input: &str) -> Result<(), Box<dyn DomainError>> {
    if input.trim().is_empty() {
        Err(Box::new(EmptyEmail))
    } else {
        Ok(())
    }
}

fn main() {
    let error = validate_email(" ").unwrap_err();
    assert_eq!(error.code(), "email.empty");
    assert_eq!(error.category(), ErrorCategory::Validation);
}
