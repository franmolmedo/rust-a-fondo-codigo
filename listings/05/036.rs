fn main() {
    let mut values = vec![String::from("uno")];
    let first_index = 0;

    values.push(String::from("dos"));
    let first = &values[first_index];

    assert_eq!(first, "uno");
}
