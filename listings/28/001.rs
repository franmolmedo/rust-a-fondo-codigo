#[test]
fn percentage_rejects_values_above_one_hundred() {
    assert!(Percentage::new(101).is_none());
}
