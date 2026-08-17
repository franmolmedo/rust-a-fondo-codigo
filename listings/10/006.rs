#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    MissingAt,
    TooLong { maximum: usize, actual: usize },
}

impl Email {
    pub fn parse(input: &str) -> Result<Self, EmailError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(EmailError::Empty);
        }
        if !input.contains('@') {
            return Err(EmailError::MissingAt);
        }

        const MAXIMUM: usize = 254;
        if input.len() > MAXIMUM {
            return Err(EmailError::TooLong {
                maximum: MAXIMUM,
                actual: input.len(),
            });
        }

        Ok(Self(input.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    let email = Email::parse("  ada@example.com ").unwrap();
    assert_eq!(email.as_str(), "ada@example.com");
    assert_eq!(Email::parse("ada"), Err(EmailError::MissingAt));
}
