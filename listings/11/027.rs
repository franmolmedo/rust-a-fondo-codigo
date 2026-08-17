use std::num::NonZeroUsize;

fn retry<T, E, F>(attempts: NonZeroUsize, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for attempt in 1..=attempts.get() {
        match operation() {
            Ok(value) => return Ok(value),
            Err(error) if attempt == attempts.get() => return Err(error),
            Err(_) => {}
        }
    }
    unreachable!("NonZeroUsize garantiza al menos un intento")
}

fn main() {
    let mut calls = 0;
    let result = retry(NonZeroUsize::new(3).unwrap(), || {
        calls += 1;
        (calls == 3).then_some("listo").ok_or("temporal")
    });

    assert_eq!(result, Ok("listo"));
    assert_eq!(calls, 3);
}
