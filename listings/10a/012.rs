#[derive(Debug, PartialEq)]
enum EmailError {
    MissingAt,
}

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

#[derive(Debug, PartialEq)]
enum RegisterUserError {
    InvalidEmail(EmailError),
    DuplicateEmail,
    Repository(RepositoryError),
}

impl From<EmailError> for RegisterUserError {
    fn from(source: EmailError) -> Self {
        Self::InvalidEmail(source)
    }
}

fn main() {
    let error = RegisterUserError::from(EmailError::MissingAt);
    assert_eq!(error, RegisterUserError::InvalidEmail(EmailError::MissingAt));
}
