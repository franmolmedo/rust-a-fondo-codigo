use course_solutions::organization::c54::{
    InMemoryEmailStore, InsertOutcome, RegistrationEvent, characterize_registration,
};
use std::sync::{Barrier, Mutex};

#[test]
fn c54_two_threads_share_one_exclusive_insert_operation() {
    let store = Mutex::new(InMemoryEmailStore::default());
    let barrier = Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let insert = || {
            barrier.wait();
            store.lock().unwrap().insert_unique("user@example.test")
        };
        let first = scope.spawn(insert);
        let second = scope.spawn(insert);
        [first.join().unwrap(), second.join().unwrap()]
    });
    assert_eq!(
        results
            .iter()
            .filter(|&&value| value == InsertOutcome::Inserted)
            .count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|&&value| value == InsertOutcome::Duplicate)
            .count(),
        1
    );
    assert_eq!(store.into_inner().unwrap().len(), 1);
}

#[test]
fn c54_validation_precedes_the_duplicate_check() {
    assert_eq!(
        characterize_registration("invalid", true),
        [RegistrationEvent::RejectedInvalid]
    );
}
