fn main() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(nested.into_iter().flatten().collect::<Vec<_>>(), [1, 2, 3, 4]);

    let optional = [Some(1), None, Some(3)];
    assert_eq!(optional.into_iter().flatten().collect::<Vec<_>>(), [1, 3]);
}
