fn length(text: &str) -> impl Copy + PartialEq<usize> {
    text.len()
}

fn main() {
    let text = String::from("rust");
    let result = length(&text);
    drop(text); // `result` may still capture the borrow in Rust 2024
    assert!(result == 4);
}
