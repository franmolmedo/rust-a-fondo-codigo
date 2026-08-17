fn words(input: &str) -> impl Iterator<Item = &str> {
    input.split_whitespace()
}

fn owned_words(input: String) -> impl Iterator<Item = String> {
    input
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .into_iter()
}

fn main() {
    assert_eq!(words("hello rust").collect::<Vec<_>>(), ["hello", "rust"]);
    assert_eq!(
        owned_words(String::from("hello rust")).collect::<Vec<_>>(),
        [String::from("hello"), String::from("rust")]
    );
}
