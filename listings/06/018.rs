fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut text = String::from("hello world");
    let first = first_word(&text).to_owned();

    text.clear();
    assert_eq!(first, "hello");
}
