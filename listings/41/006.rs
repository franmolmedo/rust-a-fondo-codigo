fn spawn_find<R>(
    repository: std::sync::Arc<R>,
    id: UserId,
) -> tokio::task::JoinHandle<Result<Option<User>, RepoError>>
where
    R: SendUserRepository + 'static,
{
    tokio::spawn(async move { repository.find(id).await })
}
