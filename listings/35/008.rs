async fn process() {
    let large = vec![0_u8; 1_000_000];
    wait_for_signal().await;
    consume(large);
}
