fn total<I>(prices: I) -> Option<u64>
where
    I: IntoIterator<Item = u64>,
{
    prices.into_iter().try_fold(0_u64, u64::checked_add)
}

fn main() {
    assert_eq!(total([100, 200]), Some(300));
    assert_eq!(total(vec![300, 400]), Some(700));
    assert_eq!(total((1..=3).map(|value| value * 10)), Some(60));
    assert_eq!(total([u64::MAX, 1]), None);
}
