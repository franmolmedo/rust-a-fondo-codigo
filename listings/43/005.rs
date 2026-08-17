async fn count_words(document: String) -> Result<usize, tokio::task::JoinError> {
    tokio::task::spawn_blocking(move || document.split_whitespace().count()).await
}
