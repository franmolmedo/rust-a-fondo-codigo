fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("Rust seguro");
    let word = first_word(&sentence);

    assert_eq!(word, "Rust");
}
