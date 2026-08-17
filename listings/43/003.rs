#[tokio::main]
async fn main() {
    tokio::task::yield_now().await;
}
