fn length_owned(text: String) -> (String, usize) {
    let length = text.len();
    (text, length)
}

fn main() {
    let text = String::from("rust");
    let (text, length) = length_owned(text);
    assert_eq!(length, 4);
    assert_eq!(text, "rust");
}
