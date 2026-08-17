fn largest<T: PartialOrd>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in &values[1..] {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}

fn main() {
    assert_eq!(largest(&[3, 9, 4]), Some(&9));
    assert_eq!(largest::<i32>(&[]), None);
}
