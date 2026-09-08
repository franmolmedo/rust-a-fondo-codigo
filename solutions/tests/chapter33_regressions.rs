use course_solutions::async_rust::c33::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn c33_supervision_distinguishes_an_aborted_task_from_a_panic() {
    let handle = tokio::spawn(std::future::pending::<()>());
    handle.abort();
    assert_eq!(supervise(handle).await, Err(SupervisionError::Cancelled));
}

#[tokio::test]
async fn c33_arguments_are_evaluated_before_the_async_body_runs() {
    let evaluated = AtomicUsize::new(0);
    let counter = Arc::new(AtomicUsize::new(usize::MAX));
    let future = lazy_effect({
        evaluated.fetch_add(1, Ordering::SeqCst);
        Arc::clone(&counter)
    });
    assert_eq!(evaluated.load(Ordering::SeqCst), 1);
    assert_eq!(counter.load(Ordering::SeqCst), usize::MAX);
    assert_eq!(future.await, usize::MAX);
    assert_eq!(counter.load(Ordering::SeqCst), usize::MAX);
}

#[test]
fn c33_a_whitespace_owner_does_not_describe_a_completion_plan() {
    assert!(!lifecycle_is_explicit(&[SubtaskPlan {
        name: "audit",
        owner: "request",
        completion: CompletionPolicy::TransferTo(" \t"),
    }]));
}
