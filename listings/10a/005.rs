#[derive(Debug)]
enum RegisterError {
    InvalidEmail,
    DuplicateEmail,
    RepositoryUnavailable,
}

fn status(error: RegisterError) -> u16 {
    match error {
        RegisterError::InvalidEmail => 400,
        RegisterError::DuplicateEmail => 409,
        RegisterError::RepositoryUnavailable => 503,
    }
}

fn main() {
    assert_eq!(status(RegisterError::DuplicateEmail), 409);
}
