fn require_identity<F>(callback: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let local = String::from("local");
    callback(&local).to_owned()
}

fn identity(value: &str) -> &str {
    value
}

fn main() {
    assert_eq!(require_identity(identity), "local");
}
