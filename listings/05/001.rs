fn length(text: &str) -> usize {
    text.len()
}

fn main() {
    let name = String::from("Ferris");

    assert_eq!(length(&name), 6);
    assert_eq!(name, "Ferris");
}
