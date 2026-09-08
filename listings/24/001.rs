async fn example() {
    let data = [0_u8; 64];
    let view = &data[..4]; // referencia a datos almacenados dentro del future
    pause().await;
    println!("{view:?}");
}
