fn length(text: &str) -> usize {
    text.len()
}

fn main() {
    let text = String::from("rust");
    assert_eq!(length(&text), 4);
    assert_eq!(text, "rust");
}
