fn main() {
    let lines = ["hello world", "rust language"];
    let words: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect();

    assert_eq!(words, ["hello", "world", "rust", "language"]);
}
