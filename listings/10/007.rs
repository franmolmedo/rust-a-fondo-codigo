#[derive(Debug, PartialEq)]
struct Email(String);

#[derive(Debug, PartialEq)]
enum EmailError {
    MissingAt,
}

impl Email {
    fn parse(input: &str) -> Result<Self, EmailError> {
        input
            .contains('@')
            .then(|| Self(input.to_owned()))
            .ok_or(EmailError::MissingAt)
    }
}

#[derive(Debug, PartialEq)]
struct User {
    email: Email,
}

#[derive(Debug, PartialEq)]
enum CreateUserError {
    InvalidEmail(EmailError),
}

fn create_user(input: &str) -> Result<User, CreateUserError> {
    let email = Email::parse(input).map_err(CreateUserError::InvalidEmail)?;
    Ok(User { email })
}

fn main() {
    assert!(create_user("ada@example.com").is_ok());
    assert_eq!(
        create_user("ada"),
        Err(CreateUserError::InvalidEmail(EmailError::MissingAt))
    );
}
