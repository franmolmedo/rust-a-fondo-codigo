fn scaled(factor: i32) -> impl Iterator<Item = i32> {
    (0..3).map(move |value| value * factor)
}

fn main() {
    assert_eq!(scaled(2).collect::<Vec<_>>(), [0, 2, 4]);
    assert_eq!(scaled(-1).collect::<Vec<_>>(), [0, -1, -2]);
}
