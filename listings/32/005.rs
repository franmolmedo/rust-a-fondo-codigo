use std::sync::atomic::{AtomicUsize, Ordering};

fn decrement_if_positive(value: &AtomicUsize) -> bool {
    value
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_sub(1)
        })
        .is_ok()
}
