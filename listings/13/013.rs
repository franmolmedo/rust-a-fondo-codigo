fn total_selected(values: &[u64]) -> u128 {
    values
        .iter()
        .copied()
        .map(|value| u128::from(value) * 2)
        .filter(|value| value % 3 == 0)
        .sum()
}

fn main() {
    assert_eq!(total_selected(&[1, 2, 3, 4, 6]), 18);
    assert_eq!(total_selected(&[u64::MAX]), u128::from(u64::MAX) * 2);
}
