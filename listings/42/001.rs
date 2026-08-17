fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split_whitespace()
}

fn main() {
    let text = String::from("uno dos");
    assert_eq!(words(&text).collect::<Vec<_>>(), ["uno", "dos"]);
}
