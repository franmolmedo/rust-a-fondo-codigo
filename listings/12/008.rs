fn doubled_evens(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .map(|value| value * 2)
        .collect()
}

fn main() {
    assert_eq!(doubled_evens(&[1, 2, 3, 4]), [4, 8]);
}
