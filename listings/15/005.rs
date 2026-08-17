use std::fmt::Display;

fn render_all<I>(items: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: Display,
{
    items.into_iter().map(|item| item.to_string()).collect()
}

fn main() {
    assert_eq!(render_all([10, 20]), ["10", "20"]);
    assert_eq!(render_all(vec!["a", "b"]), ["a", "b"]);
}
