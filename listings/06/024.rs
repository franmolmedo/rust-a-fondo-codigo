fn apply_to_texts<F>(function: F)
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let first = String::from("uno");
    assert_eq!(function(&first), 3);

    let second = String::from("cuatro");
    assert_eq!(function(&second), 6);
}

fn main() {
    apply_to_texts(str::len);
}
