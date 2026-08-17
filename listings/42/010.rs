async fn borrowed_length(text: &str) -> usize {
    tokio::task::yield_now().await;
    text.len()
}
