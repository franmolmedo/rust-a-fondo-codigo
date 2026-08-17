use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum RepoError {
    Unavailable,
}

trait UserRepository {
    fn add(&mut self, user: User) -> Result<(), RepoError>;
    fn find(&self, id: u64) -> Option<&User>;
}

#[derive(Default)]
struct InMemoryUsers {
    users: HashMap<u64, User>,
}

impl UserRepository for InMemoryUsers {
    fn add(&mut self, user: User) -> Result<(), RepoError> {
        self.users.insert(user.id, user);
        Ok(())
    }

    fn find(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }
}
