fn increment_neighbors(values: &mut [i32], middle: usize) {
    let (left, right) = values.split_at_mut(middle);
    left[middle - 1] += 1;
    right[0] += 1;
}

fn main() {
    let mut values = [10, 20, 30];
    increment_neighbors(&mut values, 1);

    assert_eq!(values, [11, 21, 30]);
}
