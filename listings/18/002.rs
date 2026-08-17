use std::fmt::Display;

fn render_pair(left: impl Display, right: impl Display) -> String {
    format!("{left} | {right}")
}

fn equal<T: PartialEq>(left: T, right: T) -> bool {
    left == right
}

fn main() {
    assert_eq!(render_pair(7, "días"), "7 | días");
    assert!(equal(String::from("Rust"), String::from("Rust")));
}
