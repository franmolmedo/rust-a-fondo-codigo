use course_solutions::concurrency::c32::{Metrics, PublishedValue, QuotaBook};
use std::sync::Barrier;
use std::thread;

#[test]
fn c32_metric_updates_saturate_instead_of_reusing_low_values() {
    let metrics = Metrics::default();
    metrics.record_requests(u64::MAX);
    metrics.record_request();
    assert_eq!(metrics.requests(), u64::MAX);
}

#[test]
fn c32_only_one_producer_can_publish_the_immutable_value() {
    let published = PublishedValue::default();
    let start = Barrier::new(2);
    let attempts = thread::scope(|scope| {
        let handles: Vec<_> = [17, 23]
            .into_iter()
            .map(|value| {
                let published = &published;
                let start = &start;
                scope.spawn(move || {
                    start.wait();
                    (value, published.publish(value))
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>()
    });
    let winners: Vec<_> = attempts
        .iter()
        .filter(|(_, result)| result.is_ok())
        .collect();
    assert_eq!(winners.len(), 1);
    assert_eq!(published.read(), Some(winners[0].0));
    assert!(published.publish(99).is_err());
    assert_eq!(published.read(), Some(winners[0].0));
}

#[test]
fn c32_one_guard_preserves_all_three_quota_fields_at_the_limit() {
    let quota = QuotaBook::new(u64::MAX);
    assert!(quota.reserve(u64::MAX));
    assert!(!quota.reserve(1));
    let snapshot = quota.snapshot();
    assert_eq!(snapshot.total, u64::MAX);
    assert_eq!(snapshot.available, 0);
    assert_eq!(snapshot.reserved, u64::MAX);
}
