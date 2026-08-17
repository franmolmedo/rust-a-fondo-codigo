fn owned_length(text: &str) -> impl std::future::Future<Output = usize> + Send + 'static {
    let owned = text.to_owned();
    async move {
        cooperative_pause().await;
        owned.len()
    }
}
