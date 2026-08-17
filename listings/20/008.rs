fn accepts_any(text: &str) -> usize {
    text.len()
}

fn main() {
    let only_receives_static: fn(&'static str) -> usize = accepts_any;
    assert_eq!(only_receives_static("Rust"), 4);
}
