#[trait_variant::make(UserRepository: Send)]
pub trait LocalUserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>, RepoError>;
}
