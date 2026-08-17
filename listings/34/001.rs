use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

trait FutureShape {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
