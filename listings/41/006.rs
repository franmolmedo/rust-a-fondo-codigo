fn spawn_find<R>(
    repository: std::sync::Arc<R>,
    id: u64,
) -> tokio::task::JoinHandle<Option<User>>
where
    R: SendUserRepository + 'static,
{
    tokio::spawn(async move { repository.find(id).await })
}
