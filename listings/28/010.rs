use std::collections::HashMap;

struct User {
    id: u64,
    name: String,
}

#[derive(Debug, PartialEq)]
enum RepoError {
    Unavailable,
}

trait UserRepository {
    fn add(&mut self, user: User) -> Result<(), RepoError>;
    fn find(&self, id: u64) -> Option<&User>;
}

#[derive(Debug)]
enum RegisterError {
    Repository(RepoError),
}

fn register(repo: &mut impl UserRepository, user: User) -> Result<(), RegisterError> {
    repo.add(user).map_err(RegisterError::Repository)
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
