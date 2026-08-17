trait UserRepository {
    fn find(
        &self,
        id: UserId,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepoError>>;
}
