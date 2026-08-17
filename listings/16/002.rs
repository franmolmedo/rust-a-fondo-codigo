fn largest<T>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in &values[1..] {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}

fn main() {}
