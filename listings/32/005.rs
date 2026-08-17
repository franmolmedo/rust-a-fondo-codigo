fn decrement_if_positive(value: &std::sync::atomic::AtomicUsize) -> bool {
    value
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_sub(1)
        })
        .is_ok()
}
