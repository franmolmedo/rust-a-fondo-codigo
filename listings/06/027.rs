#[derive(Debug, PartialEq)]
struct Email(String);

#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
}

impl Email {
    fn parse(value: String) -> Result<Self, EmailError> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        }
        if !value.contains('@') {
            return Err(EmailError::MissingAt);
        }
        Ok(Self(value))
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_inner(self) -> String {
        self.0
    }
}

fn main() {
    let email = Email::parse(String::from("ada@example.test")).unwrap();
    assert_eq!(email.as_str(), "ada@example.test");
    assert_eq!(email.into_inner(), "ada@example.test");
}
