async fn length(text: &str) -> usize {
    cooperative_pause().await;
    text.len()
}
