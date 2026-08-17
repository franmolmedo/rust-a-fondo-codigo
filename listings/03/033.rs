fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    let array = [1, 2, 3, 4];
    let vector = vec![1, 2, 3, 4];

    assert_eq!(sum(&array), 10);
    assert_eq!(sum(&vector), 10);
    assert_eq!(sum(&vector[1..3]), 5);
}
