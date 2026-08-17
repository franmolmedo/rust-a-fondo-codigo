use tokio::runtime::Handle;
use tokio::task::JoinHandle;

fn spawn_cleanup(handle: &Handle) -> JoinHandle<()> {
    handle.spawn(async {
        tokio::task::yield_now().await;
    })
}
