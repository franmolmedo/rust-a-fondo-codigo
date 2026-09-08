fn scaled(factor: i32) -> impl Iterator<Item = i64> {
    (0_i64..3).map(move |value| value * i64::from(factor))
}

fn main() {
    assert_eq!(scaled(2).collect::<Vec<_>>(), [0, 2, 4]);
    assert_eq!(scaled(-1).collect::<Vec<_>>(), [0, -1, -2]);
    assert_eq!(scaled(i32::MAX).last(), Some(i64::from(i32::MAX) * 2));
}
