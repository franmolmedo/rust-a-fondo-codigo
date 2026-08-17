fn normalized(input: &[String]) -> impl Iterator<Item = &str> {
    input
        .iter()
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn normalized_owned(input: &[String]) -> Vec<String> {
    normalized(input).map(str::to_owned).collect()
}

fn main() {
    let input = vec![String::from(" Rust "), String::from("  ")];
    assert_eq!(normalized(&input).collect::<Vec<_>>(), ["Rust"]);
    assert_eq!(normalized_owned(&input), [String::from("Rust")]);
}
