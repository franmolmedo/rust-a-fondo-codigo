#[derive(Clone, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

trait Repository {
    type Entity;
    type Error;

    fn find(&self, id: u64) -> Result<Option<Self::Entity>, Self::Error>;
}

struct InMemoryUsers {
    users: Vec<User>,
}

impl Repository for InMemoryUsers {
    type Entity = User;
    type Error = std::convert::Infallible;

    fn find(&self, id: u64) -> Result<Option<User>, Self::Error> {
        Ok(self.users.iter().find(|user| user.id == id).cloned())
    }
}

fn main() {
    let repository = InMemoryUsers {
        users: vec![User { id: 7, name: String::from("Ada") }],
    };
    assert_eq!(repository.find(7).unwrap().unwrap().name, "Ada");
    assert_eq!(repository.find(9).unwrap(), None);
}
