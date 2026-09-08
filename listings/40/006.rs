async fn invalid_overlap<F>(mut callback: F)
where
    F: AsyncFnMut(),
{
    let first = callback();
    let second = callback(); // A second mutable borrow overlaps the first.
    first.await;
    second.await;
}
