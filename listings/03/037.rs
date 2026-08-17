fn main() {
    let text = "éclair";
    let first_char = text.chars().next();
    let first_byte = text.as_bytes().first();

    assert_eq!(first_char, Some('é'));
    assert_eq!(first_byte, Some(&0xc3));
}
