#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
    TooLong { maximum: usize, actual: usize },
}

fn validate_email(input: &str) -> Result<(), EmailError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(EmailError::Empty);
    }
    if !input.contains('@') {
        return Err(EmailError::MissingAt);
    }
    if input.len() > 254 {
        return Err(EmailError::TooLong {
            maximum: 254,
            actual: input.len(),
        });
    }
    Ok(())
}

fn main() {
    assert_eq!(validate_email("ada"), Err(EmailError::MissingAt));
}
