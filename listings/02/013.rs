fn ready() -> bool {
    true
}

fn main() {
    let result = loop {
        if ready() {
            break 42;
        }
    };
    assert_eq!(result, 42);
}
