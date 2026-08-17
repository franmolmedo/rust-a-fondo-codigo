fn call_once<F, T>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    operation()
}

fn main() {
    let text = String::from("owned");
    let consume = move || text;

    assert_eq!(call_once(consume), "owned");
}
