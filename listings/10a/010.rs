fn retry_once<T, E, F>(mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    operation().or_else(|_| operation())
}

fn main() {
    let mut calls = 0;
    let result: Result<&str, &str> = retry_once(|| {
        calls += 1;
        if calls == 2 { Ok("listo") } else { Err("temporal") }
    });

    assert_eq!(result, Ok("listo"));
}
