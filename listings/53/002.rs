mod client {
    #[derive(Debug)]
    pub struct Client;
}
mod error {
    #[derive(Debug)]
    pub struct LoadError;
}
mod model {
    #[derive(Debug)]
    pub struct User;
    #[derive(Debug)]
    pub struct UserId;
}

pub use client::Client;
pub use error::LoadError;
pub use model::{User, UserId};

fn main() {
    let _ = (Client, LoadError, User, UserId);
}
