fn total<I>(prices: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    prices.into_iter().sum()
}

fn main() {
    assert_eq!(total([100, 200]), 300);
    assert_eq!(total(vec![300, 400]), 700);
    assert_eq!(total((1..=3).map(|value| value * 10)), 60);
}
