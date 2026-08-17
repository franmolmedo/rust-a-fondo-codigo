fn equals_expected(value: i32, expected: i32) -> bool {
    match value {
        candidate if candidate == expected => true,
        _ => false,
    }
}

const SPECIAL: i32 = 42;

fn main() {
    assert!(equals_expected(10, 10));
    assert!(matches!(42, SPECIAL));
}
