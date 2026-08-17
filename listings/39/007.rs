match receiver.recv().await {
    Ok(event) => apply(event),
    Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
        request_snapshot(missed).await?;
    }
    Err(tokio::sync::broadcast::error::RecvError::Closed) => return Ok(()),
}
