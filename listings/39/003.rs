let (tx, mut rx) = tokio::sync::mpsc::channel::<Job>(64);

tx.send(job).await?;
while let Some(job) = rx.recv().await {
    process(job).await;
}
