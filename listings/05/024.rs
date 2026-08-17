fn push_length(values: &mut Vec<usize>, length: usize) {
    values.push(length);
}

fn main() {
    let mut values = vec![10, 20, 30];
    let length = values.len();
    push_length(&mut values, length);

    assert_eq!(values, [10, 20, 30, 3]);
}
