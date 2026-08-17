fn main() {
    let mut text = String::from("abc");
    let other = text;

    text = String::from("xyz");

    assert_eq!(text, "xyz");
    assert_eq!(other, "abc");
}
