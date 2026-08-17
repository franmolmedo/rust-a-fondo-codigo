fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Rust seguro");
    let word = first_word(&sentence);

    assert_eq!(word, "Rust");
    assert_eq!(sentence, "Rust seguro");
}
