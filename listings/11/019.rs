fn make_operation(kind: &str) -> Box<dyn Fn(i32) -> i32> {
    match kind {
        "double" => Box::new(|value| value * 2),
        _ => Box::new(|value| value + 1),
    }
}

fn main() {
    assert_eq!(make_operation("double")(4), 8);
    assert_eq!(make_operation("increment")(4), 5);
}
