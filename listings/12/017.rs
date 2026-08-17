fn checked_sum(values: impl IntoIterator<Item = i64>) -> Option<i64> {
    values.into_iter().try_fold(0_i64, i64::checked_add)
}

fn main() {
    assert_eq!(checked_sum([1, 2, 3]), Some(6));
    assert_eq!(checked_sum([i64::MAX, 1]), None);
}
