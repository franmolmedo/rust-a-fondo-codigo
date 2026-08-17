fn length(text: &str) -> impl Copy + PartialEq<usize> + use<> {
    text.len()
}

fn main() {
    let text = String::from("rust");
    let result = length(&text);
    drop(text);
    assert!(result == 4);
}
