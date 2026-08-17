fn positive<'a>(values: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    values.iter().copied().filter(|value| *value > 0)
}

fn main() {
    let values = [-2, 4, 0, 7];
    assert_eq!(positive(&values).collect::<Vec<_>>(), [4, 7]);
}
