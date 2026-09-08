use std::sync::atomic::{AtomicU64, Ordering};

fn withdraw(balance: &AtomicU64, amount: u64) -> bool {
    balance
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_sub(amount)
        })
        .is_ok()
}
