fn total_selected(values: &[u64]) -> u64 {
    values
        .iter()
        .copied()
        .map(|value| value * 2)
        .filter(|value| value % 3 == 0)
        .sum()
}

fn main() {
    assert_eq!(total_selected(&[1, 2, 3, 4, 6]), 18);
}
