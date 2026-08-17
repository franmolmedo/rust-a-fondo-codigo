fn main() {
    let no_arguments = || 42;
    let one_argument = |value| value + 1;
    let two_arguments = |left, right| left + right;
    let block = |value: i32| -> i32 {
        let doubled = value * 2;
        doubled + 1
    };

    assert_eq!(no_arguments(), 42);
    assert_eq!(one_argument(1), 2);
    assert_eq!(two_arguments(2, 3), 5);
    assert_eq!(block(4), 9);
}
