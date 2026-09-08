// Run the blocking operation on the runtime's blocking thread pool.
let digest = tokio::task::spawn_blocking(move || hash_large_file(path)).await?;
