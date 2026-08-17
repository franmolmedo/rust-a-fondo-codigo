async fn invalid_overlap<F>(mut callback: F)
where
    F: AsyncFnMut(),
{
    let first = callback();
    let second = callback(); // segundo préstamo mutable
    first.await;
    second.await;
}
