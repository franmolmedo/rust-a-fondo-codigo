fn classify(number: i32) -> &'static str {
    match number {
        0 => "zero",
        1..=9 => "one digit",
        _ => "other",
    }
}

fn main() {
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(7), "one digit");
}
