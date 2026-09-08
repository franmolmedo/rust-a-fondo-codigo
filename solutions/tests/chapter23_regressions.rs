use course_solutions::memory::c23::{
    Account, BlockingQueue, Ledger, OwnedCounter, TransferError, detects_opposite_lock_order,
    transfer_ordered,
};

#[test]
fn c23_ledger_overflow_returns_an_error_without_changes_or_poisoning() {
    let ledger = Ledger::default();
    ledger.insert(1, 10);
    ledger.insert(2, u64::MAX);
    assert_eq!(
        ledger.transfer(1, 2, 1),
        Err(TransferError::BalanceOverflow)
    );
    assert_eq!(ledger.balance(1), Some(10));
    assert_eq!(ledger.balance(2), Some(u64::MAX));
    ledger.transfer(2, 1, 1).unwrap();
    assert_eq!(ledger.balance(1), Some(11));
}

#[test]
fn c23_ordered_transfer_validates_both_balances_before_writing() {
    let first = Account::new(1, 10);
    let second = Account::new(2, u64::MAX);
    assert_eq!(
        transfer_ordered(&first, &second, 1),
        Err(TransferError::BalanceOverflow)
    );
    assert_eq!((first.balance(), second.balance()), (10, u64::MAX));
    transfer_ordered(&second, &first, 1).unwrap();
    assert_eq!((first.balance(), second.balance()), (11, u64::MAX - 1));
}

#[test]
fn c23_opposite_orders_are_detected_without_hanging_the_test() {
    assert!(detects_opposite_lock_order());
}

#[test]
fn c23_blocking_queue_preserves_arrival_order() {
    let queue = BlockingQueue::new();
    queue.push("first");
    queue.push("second");
    assert_eq!(queue.pop_blocking(), "first");
    assert_eq!(queue.pop_blocking(), "second");
}

#[test]
fn c23_owned_metrics_counter_saturates_and_remains_available() {
    let counter = OwnedCounter::start(2);
    counter.add(u64::MAX).unwrap();
    counter.add(1).unwrap();
    assert_eq!(counter.snapshot(), Ok(u64::MAX));
    counter.shutdown().unwrap();
}
