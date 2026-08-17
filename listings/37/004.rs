use std::{future::Future, pin::Pin};

type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

fn job(input: &str) -> BoxFuture<'_, usize> {
    Box::pin(async move { input.len() })
}
