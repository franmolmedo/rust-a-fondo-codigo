fn multiplier(factor: i64) -> impl Fn(i64) -> i128 {
    move |value| i128::from(value) * i128::from(factor)
}

fn main() {
    let double = multiplier(2);
    let triple = multiplier(3);
    assert_eq!(double(21), 42);
    assert_eq!(triple(14), 42);
    assert_eq!(double(i64::MAX), i128::from(i64::MAX) * 2);
}
