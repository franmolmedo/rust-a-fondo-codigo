use course_solutions::async_rust::c40::{
    SendFuture, push_with_lending_closure, spawn_boxed_callback,
};
use std::{
    future::Future,
    task::{Context, Waker},
};

#[tokio::test]
async fn c40_cancelled_lending_call_releases_the_mutable_borrow() {
    let mut values = vec![1];
    {
        let mut operation = std::pin::pin!(push_with_lending_closure(&mut values, 2));
        let mut context = Context::from_waker(Waker::noop());
        assert!(operation.as_mut().poll(&mut context).is_pending());
    }
    assert_eq!(values, [1]);
    assert_eq!(push_with_lending_closure(&mut values, 3).await, 2);
    assert_eq!(values, [1, 3]);
}

#[tokio::test]
async fn c40_spawn_returns_a_handle_that_can_be_aborted_and_joined() {
    let handle = spawn_boxed_callback(|| Box::pin(std::future::pending()) as SendFuture<()>);
    handle.abort();
    assert!(handle.await.unwrap_err().is_cancelled());
}
