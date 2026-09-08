use course_solutions::memory::c24::Counted;
use std::cell::Cell;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

#[test]
fn c24_poll_count_saturates_without_preventing_completion() {
    let mut counted = Box::pin(Counted::new(async { 42 }));
    *counted.as_mut().project().1 = u32::MAX;
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(counted.as_mut().poll(&mut context), Poll::Ready(42));
    assert_eq!(counted.polls(), u32::MAX);
}

#[test]
fn c24_projection_preserves_the_inner_address_when_the_box_moves() {
    struct AddressFuture {
        address: Cell<Option<*const Self>>,
        _pin: PhantomPinned,
    }

    impl Future for AddressFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
            let this = self.as_ref().get_ref();
            if let Some(previous) = this.address.get() {
                assert_eq!(previous, this as *const Self);
                Poll::Ready(())
            } else {
                this.address.set(Some(this as *const Self));
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    let mut original = Box::pin(Counted::new(AddressFuture {
        address: Cell::new(None),
        _pin: PhantomPinned,
    }));
    let mut context = Context::from_waker(Waker::noop());
    assert!(original.as_mut().poll(&mut context).is_pending());
    let mut moved = original;
    assert_eq!(moved.as_mut().poll(&mut context), Poll::Ready(()));
    assert_eq!(moved.polls(), 2);
}
