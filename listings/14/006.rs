#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountryCode([u8; 2]);

#[derive(Debug, PartialEq, Eq)]
pub enum CountryCodeError {
    WrongLength,
    NotAsciiAlphabetic,
}

impl CountryCode {
    pub fn parse(raw: &str) -> Result<Self, CountryCodeError> {
        let bytes = raw.as_bytes();
        if bytes.len() != 2 {
            return Err(CountryCodeError::WrongLength);
        }
        if !bytes.iter().all(u8::is_ascii_alphabetic) {
            return Err(CountryCodeError::NotAsciiAlphabetic);
        }
        Ok(Self([
            bytes[0].to_ascii_uppercase(),
            bytes[1].to_ascii_uppercase(),
        ]))
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("invariante: dos letras ASCII")
    }
}

fn main() {
    let lower = CountryCode::parse("es").unwrap();
    let upper = CountryCode::parse("ES").unwrap();
    assert_eq!(lower, upper);
    assert_eq!(lower.as_str(), "ES");
}
