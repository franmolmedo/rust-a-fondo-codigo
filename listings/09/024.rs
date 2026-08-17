fn describe(values: &[i32]) -> String {
    match values {
        [] => String::from("empty"),
        [one] => format!("one: {one}"),
        [first, second] => format!("two: {first}, {second}"),
        [first, middle @ .., last] => {
            format!("many: {first}, {} middle, {last}", middle.len())
        }
    }
}

fn main() {
    assert_eq!(describe(&[]), "empty");
    assert_eq!(describe(&[1]), "one: 1");
    assert_eq!(describe(&[1, 2, 3, 4]), "many: 1, 2 middle, 4");
}
