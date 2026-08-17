async fn retry<F, T, E>(mut operation: F, max_attempts: usize) -> Result<T, E>
where
    F: AsyncFnMut() -> Result<T, E>,
{
    assert!(max_attempts > 0);
    let mut last_error = None;
    for _ in 0..max_attempts {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.expect("hubo al menos un intento"))
}
