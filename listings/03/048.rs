fn main() {
    let mut values = Vec::with_capacity(10);
    values.push(1);
    values.push(2);

    assert_eq!(values.len(), 2);
    assert!(values.capacity() >= 10);
}
