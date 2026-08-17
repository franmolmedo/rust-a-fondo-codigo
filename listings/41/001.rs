trait UserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>, RepoError>;
}

impl UserRepository for InMemoryUsers {
    async fn find(&self, id: UserId) -> Result<Option<User>, RepoError> {
        Ok(self.users.get(&id).cloned())
    }
}
