fn consume(text: String) -> usize {
    text.len()
}

fn main() {
    let name = String::from("Ada");
    assert_eq!(consume(name), 3);
}
