use std::fmt;

#[derive(PartialEq, Eq)]
struct PasswordHash(String);

#[derive(Debug, PartialEq, Eq)]
struct EmptyHash;

impl PasswordHash {
    fn parse(value: String) -> Result<Self, EmptyHash> {
        if value.is_empty() {
            Err(EmptyHash)
        } else {
            Ok(Self(value))
        }
    }

    fn matches_for_demo(&self, candidate: &str) -> bool {
        self.0 == candidate
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PasswordHash([REDACTED])")
    }
}

fn main() {
    let hash = PasswordHash::parse(String::from("secret-hash")).unwrap();
    assert!(hash.matches_for_demo("secret-hash"));
    assert_eq!(format!("{hash:?}"), "PasswordHash([REDACTED])");
}
