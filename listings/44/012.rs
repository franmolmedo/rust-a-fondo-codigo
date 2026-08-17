fn checked_get<T>(values: &[T], index: usize) -> Option<&T> {
    if index < values.len() {
        // SAFETY: this branch proves the exact precondition of `get_unchecked`.
        Some(unsafe { values.get_unchecked(index) })
    } else {
        None
    }
}

fn main() {
    assert_eq!(checked_get(&[10, 20], 1), Some(&20));
    assert_eq!(checked_get(&[10, 20], 2), None);
}
