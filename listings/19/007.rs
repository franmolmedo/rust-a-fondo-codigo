fn sum(iterator: &mut dyn Iterator<Item = i32>) -> Option<i32> {
    let mut total = 0_i32;
    for value in iterator {
        total = total.checked_add(value)?;
    }
    Some(total)
}

fn main() {
    let mut values = vec![2, 3, 5].into_iter();
    assert_eq!(sum(&mut values), Some(10));
    assert_eq!(sum(&mut [i32::MAX, 1].into_iter()), None);
    assert_eq!(sum(&mut std::iter::empty()), Some(0));
}
