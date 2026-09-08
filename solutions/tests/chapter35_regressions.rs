use course_solutions::async_rust::c35::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn c35_the_parsing_example_accepts_the_whole_u64_range() {
    let input = u64::MAX.to_string();
    let (_, parsed, validated) = infer_three_outputs(&input).await;
    assert_eq!(parsed, Ok(u64::MAX));
    assert_eq!(validated, Ok(()));
    assert!(infer_three_outputs("18446744073709551616").await.1.is_err());
}

#[test]
fn c35_dropping_before_the_first_poll_constructs_no_local_guards() {
    let constructed = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));
    drop(pending_with_drop_probe(
        Arc::clone(&constructed),
        Arc::clone(&dropped),
    ));
    assert_eq!(constructed.load(Ordering::SeqCst), 0);
    assert_eq!(dropped.load(Ordering::SeqCst), 0);
    assert_eq!(Arc::strong_count(&constructed), 1);
    assert_eq!(Arc::strong_count(&dropped), 1);
}

#[test]
fn c35_an_empty_inventory_has_no_dominant_state() {
    assert_eq!(dominant_suspension(&[]), None);
}
