trait SendUserRepository: Send + Sync {
    fn find(
        &self,
        id: UserId,
    ) -> impl std::future::Future<Output = Result<Option<User>, RepoError>> + Send;
}
