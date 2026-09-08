use course_solutions::memory::c22::Counter;

#[test]
fn c22_counter_saturates_at_its_declared_limit() {
    let counter = Counter::new(u64::MAX - 1);
    counter.increment();
    assert_eq!(counter.value(), u64::MAX);
    counter.increment();
    assert_eq!(counter.value(), u64::MAX);
}

mod book_event_log {
    include!("../../listings/22/010.rs");

    #[test]
    fn c22_event_conversion_can_reenter_before_the_mutable_borrow() {
        struct ReentrantEvent<'a>(&'a EventLog);

        impl From<ReentrantEvent<'_>> for String {
            fn from(event: ReentrantEvent<'_>) -> Self {
                event.0.record("converted");
                String::from("outer")
            }
        }

        let log = EventLog::new();
        log.record(ReentrantEvent(&log));
        assert_eq!(log.snapshot(), ["converted", "outer"]);
    }
}
