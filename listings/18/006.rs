fn multiplier(factor: i64) -> impl Fn(i64) -> i64 {
    move |value| value * factor
}

fn main() {
    let double = multiplier(2);
    let triple = multiplier(3);
    assert_eq!(double(21), 42);
    assert_eq!(triple(14), 42);
}
