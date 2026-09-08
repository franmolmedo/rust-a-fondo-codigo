use course_solutions::projects::{
    p01_domain_ids as ids, p05_async_crawler as crawler, p07_catalog_desktop as desktop,
    p08_native_checksum as native,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn c57_id_parse_distinguishes_overflow_from_invalid_syntax() {
    assert_eq!(
        "18446744073709551616".parse::<ids::UserId>(),
        Err(ids::IdError::OutOfRange)
    );
    assert_eq!(
        "not-a-number".parse::<ids::UserId>(),
        Err(ids::IdError::NotANumber)
    );
    assert_eq!("0".parse::<ids::UserId>(), Err(ids::IdError::Zero));
    assert_eq!(
        u64::MAX.to_string().parse::<ids::UserId>().unwrap().get(),
        u64::MAX
    );
}

#[test]
fn c57_old_completion_cannot_finish_a_newer_retry() {
    let mut crawl = crawler::CrawlCoordinator::new(1, 1);
    assert_eq!(crawl.enqueue("a"), crawler::Enqueue::Accepted);
    let first = crawl.start_next().unwrap();
    assert!(crawl.finish(&first, crawler::Outcome::RetryableFailure));
    let retry = crawl.start_next().unwrap();
    assert!(!crawl.finish(&first, crawler::Outcome::Success));
    assert_eq!(crawl.in_flight(), 1);
    assert!(crawl.finished().is_empty());
    assert!(crawl.finish(&retry, crawler::Outcome::Success));
    assert!(crawl.is_drained());
    assert!(!crawl.finish(&retry, crawler::Outcome::Success));
}

#[test]
fn c57_maximum_retry_budget_does_not_wrap_the_attempt_counter() {
    let mut crawl = crawler::CrawlCoordinator::new(1, u8::MAX);
    crawl.enqueue("a");
    crawl.close_admission();
    for expected in 0..=u8::MAX {
        let request = crawl.start_next().unwrap();
        assert_eq!(request.attempt, expected);
        assert!(crawl.finish(&request, crawler::Outcome::RetryableFailure));
    }
    assert!(crawl.is_drained());
    assert_eq!(crawl.finished().len(), 1);
}

#[test]
fn c57_unknown_internal_error_does_not_recommend_retry() {
    assert!(!desktop::to_ipc_error(desktop::ApplicationError::Internal).retryable);
    assert!(!desktop::to_ipc_error(desktop::ApplicationError::InvalidInput).retryable);
    assert!(desktop::to_ipc_error(desktop::ApplicationError::Unavailable).retryable);
}

#[test]
fn c57_firewall_does_not_run_an_untrusted_panic_payload_destructor() {
    struct Payload(Arc<AtomicUsize>);
    impl Drop for Payload {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
            panic!("payload destructor must not run at the boundary");
        }
    }
    let drops = Arc::new(AtomicUsize::new(0));
    let payload = Payload(Arc::clone(&drops));
    assert_eq!(
        native::callback_firewall(|| std::panic::panic_any(payload)),
        Err(native::CallbackPanicked)
    );
    assert_eq!(drops.load(Ordering::SeqCst), 0);
}
