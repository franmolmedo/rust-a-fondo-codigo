#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UserId(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserIdError {
    Empty,
    InvalidPrefix,
}

impl UserId {
    pub fn parse(input: impl Into<String>) -> Result<Self, UserIdError> {
        let input = input.into();
        if input.is_empty() {
            return Err(UserIdError::Empty);
        }
        if !input.starts_with("usr_") {
            return Err(UserIdError::InvalidPrefix);
        }
        Ok(Self(input))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    let id = UserId::parse("usr_42").unwrap();
    assert_eq!(id.as_str(), "usr_42");
    assert_eq!(UserId::parse(""), Err(UserIdError::Empty));
}
