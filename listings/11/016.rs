fn normalize(input: &str) -> String {
    input.trim().to_lowercase()
}

fn apply<F>(input: &str, operation: F) -> String
where
    F: Fn(&str) -> String,
{
    operation(input)
}

fn main() {
    assert_eq!(apply(" Rust ", normalize), "rust");

    let pointer: fn(&str) -> String = normalize;
    assert_eq!(pointer(" BOOK "), "book");
}
