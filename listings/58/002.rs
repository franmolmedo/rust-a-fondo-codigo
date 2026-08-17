async fn execute<R>(
    repo: &R,
    command: Command,
) -> Result<Event, ExecuteError>
where
    R: Repository + Sync,
