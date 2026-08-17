use std::fmt::Display;

fn surround<T>(value: &T) -> String
where
    T: Display,
{
    format!("<{value}>")
}

fn main() {
    assert_eq!(surround(&42), "<42>");
    assert_eq!(surround(&"Rust"), "<Rust>");
}
