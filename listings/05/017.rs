fn zero_all(values: &mut [i32]) {
    for value in values {
        *value = 0;
    }
}

fn main() {
    let mut values = vec![1, 2, 3];
    zero_all(&mut values);
    assert_eq!(values, [0, 0, 0]);
}
