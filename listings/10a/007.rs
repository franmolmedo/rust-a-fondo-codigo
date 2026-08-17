use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct EmptyEmail;

impl fmt::Display for EmptyEmail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "email vacío")
    }
}

impl Error for EmptyEmail {}

fn main() {
    let error: Box<dyn Error> = Box::new(EmptyEmail);
    assert!(error.downcast_ref::<EmptyEmail>().is_some());
}
