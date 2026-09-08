use course_solutions::abstraction::c16::{
    Cents, Increase, OperationKind, Scale, apply_closed, apply_dynamic, apply_static, twice,
};

#[test]
fn c16_dispatch_strategies_agree_at_integer_boundaries() {
    for value in [i64::MIN, -1, 0, 1, i64::MAX] {
        for operand in [i64::MIN, -1, 0, 1, i64::MAX] {
            let sum = i128::from(value) + i128::from(operand);
            let product = i128::from(value) * i128::from(operand);
            assert_eq!(apply_static(&Increase(operand), value), sum);
            assert_eq!(apply_dynamic(&Increase(operand), value), sum);
            assert_eq!(apply_closed(OperationKind::Increase(operand), value), sum);
            assert_eq!(apply_static(&Scale(operand), value), product);
            assert_eq!(apply_dynamic(&Scale(operand), value), product);
            assert_eq!(apply_closed(OperationKind::Scale(operand), value), product);
        }
    }
}

#[test]
fn c16_cents_addition_accepts_the_largest_representable_sum() {
    assert_eq!(Cents(u64::MAX - 1) + Cents(1), Cents(u64::MAX));
    assert_eq!(twice(Cents(u64::MAX / 2)), Cents(u64::MAX - 1));
}

#[test]
#[should_panic(expected = "amount overflow")]
fn c16_cents_addition_rejects_overflow_explicitly() {
    let _ = twice(Cents(u64::MAX));
}
