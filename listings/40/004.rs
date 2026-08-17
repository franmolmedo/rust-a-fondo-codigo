async fn shared<F>(callback: &F) -> usize
where
    F: AsyncFn() -> usize,
{
    callback().await
}

async fn mutable<F>(callback: &mut F) -> usize
where
    F: AsyncFnMut() -> usize,
{
    callback().await
}

async fn once<F>(callback: F) -> usize
where
    F: AsyncFnOnce() -> usize,
{
    callback().await
}
