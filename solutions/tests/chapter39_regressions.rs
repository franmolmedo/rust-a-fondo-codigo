use course_solutions::async_rust::c39::{
    CapacityError, CapacityInput, CapacityPlan, plan_bounded_stage, reserve_then_build,
};
use std::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Waker},
};

#[test]
fn c39_rounding_capacity_does_not_overflow_before_division() {
    let result = plan_bounded_stage(CapacityInput {
        peak_per_second: u64::MAX,
        service_per_second: 0,
        burst_millis: 1,
        bytes_per_item: 1,
        memory_budget_bytes: u64::MAX,
        max_in_flight: 1,
    });
    let expected = usize::try_from(u64::MAX.div_ceil(1_000))
        .map(|queue_capacity| CapacityPlan {
            queue_capacity,
            max_in_flight: 1,
        })
        .map_err(|_| CapacityError::ArithmeticOverflow);
    assert_eq!(result, expected);
}

#[tokio::test]
async fn c39_cancelled_admission_does_not_build_a_message() {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
    sender.send(1).await.unwrap();
    let built = AtomicBool::new(false);
    {
        let operation = reserve_then_build(&sender, || {
            built.store(true, Ordering::Relaxed);
            2
        });
        let mut operation = std::pin::pin!(operation);
        let mut context = Context::from_waker(Waker::noop());
        assert!(operation.as_mut().poll(&mut context).is_pending());
    }
    assert!(!built.load(Ordering::Relaxed));
    assert_eq!(receiver.recv().await, Some(1));
    assert_eq!(sender.capacity(), 1);
}
