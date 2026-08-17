fn identity(value: &str) -> &str {
    value
}

fn apply<F>(callback: F, value: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    callback(value)
}

fn main() {
    let local = String::from("Rust");
    assert_eq!(apply(identity, &local), "Rust");
}
