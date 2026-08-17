#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

#[derive(Debug, PartialEq, Eq)]
pub enum UsernameError {
    Empty,
    TooLong { max: usize, actual: usize },
}

impl Username {
    pub const MAX_LEN: usize = 20;

    pub fn parse(raw: &str) -> Result<Self, UsernameError> {
        let value = raw.trim();
        if value.is_empty() {
            return Err(UsernameError::Empty);
        }
        if value.chars().count() > Self::MAX_LEN {
            return Err(UsernameError::TooLong {
                max: Self::MAX_LEN,
                actual: value.chars().count(),
            });
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

fn main() {
    let name = Username::parse("  Ferris  ").unwrap();
    assert_eq!(name.as_str(), "Ferris");
    assert_eq!(name.into_string(), "Ferris");
}
