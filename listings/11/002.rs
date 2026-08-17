fn apply<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(value)
}

fn main() {
    let increment = |value| value + 1;
    let double = |value| value * 2;

    assert_eq!(apply(10, increment), 11);
    assert_eq!(apply(10, double), 20);
}
