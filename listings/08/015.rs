#[derive(Debug, PartialEq, Eq)]
enum UsernameError {
    Empty,
    TooShort { minimum: usize, actual: usize },
    TooLong { maximum: usize, actual: usize },
    InvalidCharacter { index: usize, character: char },
}

fn validate_username(value: &str) -> Result<(), UsernameError> {
    if value.is_empty() {
        return Err(UsernameError::Empty);
    }
    let length = value.chars().count();
    if length < 3 {
        return Err(UsernameError::TooShort {
            minimum: 3,
            actual: length,
        });
    }
    if length > 20 {
        return Err(UsernameError::TooLong {
            maximum: 20,
            actual: length,
        });
    }
    if let Some((index, character)) = value
        .char_indices()
        .find(|(_, character)| !character.is_alphanumeric())
    {
        return Err(UsernameError::InvalidCharacter { index, character });
    }
    Ok(())
}

fn main() {
    assert_eq!(
        validate_username("a!"),
        Err(UsernameError::TooShort { minimum: 3, actual: 2 })
    );
    assert_eq!(
        validate_username("abcdefghijklmnopqrstu"),
        Err(UsernameError::TooLong {
            maximum: 20,
            actual: 21,
        })
    );
    assert_eq!(validate_username("ada"), Ok(()));
    assert_eq!(
        validate_username("éa!"),
        Err(UsernameError::InvalidCharacter { index: 3, character: '!' })
    );
}
