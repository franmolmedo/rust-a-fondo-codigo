use course_solutions::async_rust::c38::{Command, Deadline, run_draining_worker, with_timeout};
use std::time::Duration;

#[tokio::test]
async fn c38_shutdown_drains_a_previously_reserved_send() {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);
    let permit = sender.reserve().await.unwrap();
    sender.send(Command::Shutdown).await.unwrap();
    let worker = tokio::spawn(run_draining_worker(receiver));
    sender.closed().await;
    assert!(!worker.is_finished());
    permit.send(Command::Store(42));
    assert_eq!(worker.await.unwrap(), [42]);
}

#[tokio::test(start_paused = true)]
async fn c38_timeout_can_borrow_a_future_without_destroying_it() {
    let operation = async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        Ok::<_, ()>(42)
    };
    let mut operation = std::pin::pin!(operation);
    assert!(
        with_timeout(Duration::from_secs(1), operation.as_mut())
            .await
            .is_err()
    );
    assert_eq!(operation.await, Ok(42));
}

#[test]
fn c38_unrepresentable_deadline_is_an_error_not_a_panic() {
    assert!(Deadline::after(Duration::MAX).is_err());
}
