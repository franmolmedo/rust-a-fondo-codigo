async fn countdown(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        countdown(n - 1).await + 1
        // error[E0733]: recursion in an async fn requires boxing
    }
}
