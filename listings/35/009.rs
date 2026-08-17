async fn smaller() {
    let checksum = {
        let large = build_buffer();
        let checksum = calculate_checksum(&large);
        consume(large);
        checksum
    };

    wait_for_signal().await;
    publish(checksum);
}
