fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn main() {
    let text = String::from("\n Rust \n\n ownership ");
    assert_eq!(non_empty_lines(&text).collect::<Vec<_>>(), ["Rust", "ownership"]);
}
