fn sum(iterator: &mut dyn Iterator<Item = i32>) -> i32 {
    iterator.sum()
}

fn main() {
    let mut values = vec![2, 3, 5].into_iter();
    assert_eq!(sum(&mut values), 10);
}
