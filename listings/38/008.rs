let handle = tokio::spawn(async move { run_worker().await });
let output = handle.await?;
