// El worker async queda libre mientras el pool bloqueante calcula:
let digest = tokio::task::spawn_blocking(move || hash_large_file(path)).await?;
