#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    InvalidShape,
}

impl Email {
    pub fn parse(raw: impl Into<String>) -> Result<Self, EmailError> {
        let normalized = raw.into().trim().to_ascii_lowercase();
        if normalized.is_empty() {
            return Err(EmailError::Empty);
        }
        let valid = normalized
            .split_once('@')
            .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));
        valid.then_some(Self(normalized)).ok_or(EmailError::InvalidShape)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn welcome_recipient(email: &Email) -> String {
    format!("Bienvenida para {}", email.as_str())
}

fn main() {
    let email = Email::parse(" ADA@Example.com ").unwrap();
    assert_eq!(email.as_str(), "ada@example.com");
    assert_eq!(welcome_recipient(&email), "Bienvenida para ada@example.com");
    assert_eq!(Email::parse("sin-arroba"), Err(EmailError::InvalidShape));
}
