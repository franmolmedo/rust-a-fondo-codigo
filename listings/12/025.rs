fn total<I>(items: I) -> Option<i64>
where
    I: IntoIterator<Item = i64>,
{
    items.into_iter().try_fold(0_i64, i64::checked_add)
}

fn main() {
    assert_eq!(total([1, 2, 3]), Some(6));
    assert_eq!(total(vec![4, 5]), Some(9));
    assert_eq!(total((1..=4).map(i64::from)), Some(10));
    assert_eq!(total([i64::MAX, 1]), None);
}
