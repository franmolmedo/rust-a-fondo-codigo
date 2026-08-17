fn main() {
    let condition = true;
    let number = if condition { 1 } else { 2 };
    assert_eq!(number, 1);
}
