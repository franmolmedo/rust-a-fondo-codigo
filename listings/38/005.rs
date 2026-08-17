loop {
    tokio::select! {
        maybe = rx.recv() => match maybe {
            Some(command) => process(command).await?,
            None => break,
        },
        _ = shutdown.changed() => break,
    }
}
