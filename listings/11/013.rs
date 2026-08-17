fn unwrap_or_else<T, F>(value: Option<T>, fallback: F) -> T
where
    F: FnOnce() -> T,
{
    match value {
        Some(value) => value,
        None => fallback(),
    }
}

fn main() {
    let fallback = String::from("anonymous");
    let name = unwrap_or_else(None, || fallback);
    assert_eq!(name, "anonymous");
}
