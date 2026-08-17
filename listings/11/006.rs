fn call_twice<F>(operation: F) -> (usize, usize)
where
    F: Fn() -> usize,
{
    (operation(), operation())
}

fn main() {
    let name = String::from("Ada");
    let length = || name.len();

    assert_eq!(call_twice(length), (3, 3));
    assert_eq!(name, "Ada");
}
