fn parse_positive(input: &str) -> Option<u32> {
    let value = input.parse::<u32>().ok()?;
    (value > 0).then_some(value)
}

fn main() {
    assert_eq!(parse_positive("7"), Some(7));
    assert_eq!(parse_positive("0"), None);
    assert_eq!(parse_positive("abc"), None);
}
