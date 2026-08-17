use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
struct Draft;

#[derive(Debug, PartialEq, Eq)]
struct Verified;

#[derive(Debug, PartialEq, Eq)]
struct Registration<State> {
    email: String,
    state: PhantomData<State>,
}

#[derive(Debug, PartialEq, Eq)]
enum VerifyError {
    InvalidToken,
}

impl Registration<Draft> {
    fn new(email: impl Into<String>) -> Self {
        Self { email: email.into(), state: PhantomData }
    }

    fn verify(
        self,
        token: &str,
    ) -> Result<Registration<Verified>, (Self, VerifyError)> {
        if token != "known-token" {
            return Err((self, VerifyError::InvalidToken));
        }
        Ok(Registration { email: self.email, state: PhantomData })
    }
}

fn main() {
    let draft = Registration::<Draft>::new("ada@example.com");
    let (draft, error) = draft.verify("wrong").unwrap_err();
    assert_eq!(error, VerifyError::InvalidToken);
    let verified = draft.verify("known-token").unwrap();
    assert_eq!(verified.email, "ada@example.com");
}
