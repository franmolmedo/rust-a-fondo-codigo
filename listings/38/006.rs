let result = tokio::time::timeout(
    std::time::Duration::from_secs(2),
    load_user(id),
)
.await;
