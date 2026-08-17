async fn example() {
    let data = [0_u8; 64];
    let view = &data[..4]; // referencia a datos que viven EN el future
    pause().await;
    println!("{view:?}");
}
