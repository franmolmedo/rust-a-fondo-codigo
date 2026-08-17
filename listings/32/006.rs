fn withdraw(balance: &AtomicU64, amount: u64) -> bool {
    balance
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            current.checked_sub(amount)
        })
        .is_ok()
}
