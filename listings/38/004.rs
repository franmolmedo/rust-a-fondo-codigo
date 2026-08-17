tokio::select! {
    result = receive_command() => handle(result)?,
    _ = shutdown_signal() => begin_shutdown(),
}
