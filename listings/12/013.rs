fn first_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn main() {
    let values = [1, 3, 4, 8];
    assert_eq!(first_even(&values), Some(4));
    assert!(values.iter().any(|value| *value > 5));
    assert!(values.iter().all(|value| *value > 0));
}
