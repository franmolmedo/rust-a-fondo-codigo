async fn visit_all<F>(items: &[String], visitor: F) -> Vec<usize>
where
    F: for<'a> AsyncFn(&'a str) -> usize,
{
    let mut outputs = Vec::with_capacity(items.len());
    for item in items {
        outputs.push(visitor(item).await);
    }
    outputs
}
