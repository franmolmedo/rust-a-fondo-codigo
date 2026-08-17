fn transform<T, U, F>(value: T, operation: F) -> U
where
    F: FnOnce(T) -> U,
{
    operation(value)
}

fn main() {
    let suffix = String::from("!");
    let result = transform(String::from("hola"), |mut text| {
        text.push_str(&suffix);
        text
    });
    assert_eq!(result, "hola!");
}
