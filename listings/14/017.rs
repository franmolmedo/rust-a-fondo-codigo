#[derive(Debug, PartialEq, Eq)]
struct Email(String);

impl TryFrom<String> for Email {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.contains('@').then_some(Self(value)).ok_or(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Age(u8);

impl TryFrom<u8> for Age {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        (value >= 18).then_some(Self(value)).ok_or(())
    }
}

struct RawSignup {
    email: String,
    age: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Signup {
    email: Email,
    age: Age,
}

#[derive(Debug, PartialEq, Eq)]
enum SignupError {
    Email,
    Age,
}

impl TryFrom<RawSignup> for Signup {
    type Error = SignupError;

    fn try_from(raw: RawSignup) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::try_from(raw.email).map_err(|_| SignupError::Email)?,
            age: Age::try_from(raw.age).map_err(|_| SignupError::Age)?,
        })
    }
}

fn main() {
    let signup = Signup::try_from(RawSignup {
        email: String::from("ada@example.com"),
        age: 36,
    });
    assert!(signup.is_ok());
    assert_eq!(
        Signup::try_from(RawSignup { email: String::from("bad"), age: 36 }),
        Err(SignupError::Email)
    );
}
