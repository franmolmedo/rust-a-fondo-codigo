fn first_even(values: &[i32]) -> Option<i32> {
    let mut result = None;
    let mut index = 0;

    while index < values.len() {
        if values[index] % 2 == 0 {
            result = Some(values[index]);
            break;
        }
        index += 1;
    }

    result
}

fn main() {
    assert_eq!(first_even(&[1, 7, 4, 8]), Some(4));
    assert_eq!(first_even(&[1, 7]), None);
}
