fn with_local<F>(callback: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let local = String::from("interno");
    callback(&local)
}

fn length(value: &str) -> usize {
    value.len()
}

fn main() {
    assert_eq!(with_local(length), 7);
    assert_eq!(with_local(|value: &str| value.chars().count()), 7);
}
