use std::time::Duration;
use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()
        .unwrap();

    runtime.block_on(async {
        let started = tokio::time::Instant::now();
        tokio::time::sleep(Duration::from_secs(60)).await;
        assert_eq!(started.elapsed(), Duration::from_secs(60));
    });
}
