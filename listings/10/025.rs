#[derive(Debug, PartialEq)]
enum SignupError {
    TooYoung { minimum: u8, actual: u8 },
}

fn signup(age: u8) -> Result<(), SignupError> {
    const MINIMUM: u8 = 18;
    if age < MINIMUM {
        Err(SignupError::TooYoung {
            minimum: MINIMUM,
            actual: age,
        })
    } else {
        Ok(())
    }
}

fn main() {
    assert_eq!(
        signup(16),
        Err(SignupError::TooYoung {
            minimum: 18,
            actual: 16,
        })
    );
    assert!(matches!(signup(17), Err(SignupError::TooYoung { .. })));
}
