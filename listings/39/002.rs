while let Some(event) = stream.next().await {
    handle(event).await?;
}
