async fn countdown(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        Box::pin(countdown(n - 1)).await + 1
    }
}
