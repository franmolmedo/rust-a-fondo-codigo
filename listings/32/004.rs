use std::sync::atomic::{AtomicU64, Ordering};

fn withdraw_broken(balance: &AtomicU64, amount: u64) -> bool {
    if balance.load(Ordering::Acquire) >= amount {
        // Otro thread puede pasar esta misma comprobación aquí,
        // antes de que restemos.
        balance.fetch_sub(amount, Ordering::AcqRel);
        true
    } else {
        false
    }
}
