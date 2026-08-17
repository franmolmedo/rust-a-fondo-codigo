fn main() {
    let mut values = vec![2, 4, 6];

    let sum = {
        let view = &values;
        view.iter().sum::<i32>()
    };

    values.push(sum);
    assert_eq!(values, [2, 4, 6, 12]);
}
