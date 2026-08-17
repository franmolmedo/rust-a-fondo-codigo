async fn bad_read() -> std::io::Result<String> {
    std::fs::read_to_string("large.txt")
}
