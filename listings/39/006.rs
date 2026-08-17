struct Lookup {
    id: UserId,
    reply: tokio::sync::oneshot::Sender<Result<User, LookupError>>,
}
