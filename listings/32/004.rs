use std::sync::atomic::{AtomicU64, Ordering};

fn withdraw_broken(balance: &AtomicU64, amount: u64) -> bool {
    if balance.load(Ordering::Relaxed) >= amount {
        // Another thread can pass the same check before this subtraction.
        balance.fetch_sub(amount, Ordering::Relaxed);
        true
    } else {
        false
    }
}
