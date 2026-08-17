async fn first_line(input: &str) -> Option<&str> {
    cooperative_pause().await;
    input.lines().next()
}
