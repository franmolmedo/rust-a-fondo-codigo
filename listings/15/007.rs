use std::error::Error;
use std::fmt;

trait DomainError: Error + Send + Sync + 'static {}

impl<T> DomainError for T
where
    T: Error + Send + Sync + 'static,
{}

#[derive(Debug)]
struct MissingUser(u64);

impl fmt::Display for MissingUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no existe el usuario {}", self.0)
    }
}

impl Error for MissingUser {}

fn erase(error: impl DomainError) -> Box<dyn DomainError> {
    Box::new(error)
}

fn main() {
    assert_eq!(erase(MissingUser(7)).to_string(), "no existe el usuario 7");
}
