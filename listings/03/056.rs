fn main() {
    let text: &str = "rust";
    let numbers: &[u16] = &[10, 20, 30];

    assert_eq!(std::mem::size_of_val(text), 4);
    assert_eq!(std::mem::size_of_val(numbers), 6);
}
