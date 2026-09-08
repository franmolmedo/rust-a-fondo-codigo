use course_solutions::concurrency::c31::*;
use std::cell::Cell;
use std::sync::{Arc, Mutex, MutexGuard};

#[test]
fn c31_sync_does_not_require_send() {
    assert_sync_type::<MutexGuard<'static, u64>>();
    let shared_only = SendSafetyAudit {
        unique_ownership: true,
        thread_agnostic_resource: false,
        single_drop: true,
        synchronized_shared_access: true,
    };
    assert!(!justifies_send(shared_only));
    assert!(justifies_sync(shared_only));
}

#[test]
fn c31_the_shared_cell_metric_has_an_explicit_limit() {
    let state = Arc::new(Mutex::new(Cell::new(u32::MAX)));
    assert_eq!(increment_cell_behind_mutex(state), u32::MAX);
}

#[tokio::test]
async fn c31_both_mutex_examples_use_the_same_overflow_policy() {
    let state = Arc::new(Mutex::new(u64::MAX));
    assert_eq!(
        tokio::spawn(increment_then_yield(state)).await.unwrap(),
        u64::MAX
    );
    let state = Arc::new(tokio::sync::Mutex::new(u64::MAX));
    assert_eq!(
        tokio::spawn(increment_with_async_mutex(state))
            .await
            .unwrap(),
        u64::MAX
    );
}
