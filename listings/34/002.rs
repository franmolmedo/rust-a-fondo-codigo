use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct Immediate<T>(Option<T>);

impl<T: Unpin> Future for Immediate<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("future sondeado tras completar"))
    }
}
