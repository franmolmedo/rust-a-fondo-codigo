fn total<I>(items: I) -> i64
where
    I: IntoIterator<Item = i64>,
{
    items.into_iter().sum()
}

fn main() {
    assert_eq!(total([1, 2, 3]), 6);
    assert_eq!(total(vec![4, 5]), 9);
    assert_eq!(total((1..=4).map(i64::from)), 10);
}
