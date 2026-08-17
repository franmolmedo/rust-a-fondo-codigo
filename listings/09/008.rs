fn main() {
    let expected = 10;
    let value = 20;

    let captured = match value {
        expected => expected,
    };

    assert_eq!(captured, 20);
    assert_eq!(expected, 10);
}
