use course_solutions::async_rust::c36::*;
use std::future::Future;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

#[tokio::test]
async fn c36_revision_exhaustion_preserves_the_state_without_poisoning() {
    let initial = VersionedState {
        revision: u64::MAX,
        prepared: false,
        finished: true,
    };
    let shared = Mutex::new(initial);
    assert_eq!(
        update_after_notification(&shared, std::future::ready(())).await,
        Err(RevisionExhausted)
    );
    assert_eq!(*shared.lock().unwrap(), initial);
}

#[test]
fn c36_a_new_pending_revision_does_not_inherit_finished_from_the_old_one() {
    let shared = Mutex::new(VersionedState {
        revision: 1,
        prepared: true,
        finished: true,
    });
    let mut future = Box::pin(update_after_notification(&shared, std::future::pending()));
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Pending
    );
    assert_eq!(
        *shared.lock().unwrap(),
        VersionedState {
            revision: 2,
            prepared: true,
            finished: false
        }
    );
    drop(future);
    assert!(!shared.lock().unwrap().finished);
}
