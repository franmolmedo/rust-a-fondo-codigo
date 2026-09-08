fn doubled_evens(values: &[i32]) -> Vec<i64> {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .map(|value| i64::from(value) * 2)
        .collect()
}

fn main() {
    assert_eq!(doubled_evens(&[1, 2, 3, 4]), [4, 8]);
    assert_eq!(doubled_evens(&[i32::MIN]), [i64::from(i32::MIN) * 2]);
}
