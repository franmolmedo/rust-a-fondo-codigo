use course_solutions::async_rust::c34::{Map, OneShotEvent};
use std::future::Future;
use std::task::{Context, Poll, Waker};

#[test]
fn c34_a_signalled_event_remains_ready_for_later_waiters() {
    let event = OneShotEvent::new();
    assert!(event.signal());
    assert!(!event.signal());
    let mut future = Box::pin(event.wait());
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(())
    );
    assert_eq!(event.waiting_count(), 0);
}

#[test]
fn c34_map_does_not_invoke_its_closure_after_completion() {
    let mut future = Box::pin(Map::new(std::future::ready(21), |value| value * 2));
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(42));
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            future.as_mut().poll(&mut context)
        }))
        .is_err()
    );
}
