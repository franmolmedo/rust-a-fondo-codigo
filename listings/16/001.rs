fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}

#[derive(Debug, PartialEq, Eq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

fn main() {
    assert_eq!(first(&[10, 20]), Some(&10));
    assert_eq!(first(&[String::from("Rust")]).map(String::as_str), Some("Rust"));
    assert_eq!(Pair::new('a', 'b').left, 'a');
}
