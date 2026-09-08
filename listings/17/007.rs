use std::fmt::Display;

fn render_items<I>(items: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: Display,
{
    items.into_iter().map(|item| item.to_string()).collect()
}

fn sum_exact<I>(items: I) -> Option<u64>
where
    I: IntoIterator<Item = u64>,
{
    items.into_iter().try_fold(0_u64, u64::checked_add)
}

fn main() {
    assert_eq!(render_items([10, 20]), ["10", "20"]);
    assert_eq!(sum_exact([10, 20]), Some(30));
    assert_eq!(sum_exact([u64::MAX, 1]), None);
}
