fn main() {
    let mut seen = Vec::new();
    let result: Vec<_> = [1, 2, 3, 4]
        .into_iter()
        .inspect(|value| seen.push(*value))
        .filter(|value| value % 2 == 0)
        .collect();

    assert_eq!(seen, [1, 2, 3, 4]);
    assert_eq!(result, [2, 4]);
}
