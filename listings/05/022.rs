fn main() {
    let mut values = vec![10, 20, 30];
    values.push(values.len());

    assert_eq!(values, [10, 20, 30, 3]);
}
