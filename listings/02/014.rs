fn main() {
    let mut attempts = 0;

    let value = loop {
        attempts += 1;
        if attempts == 3 {
            break attempts * 10;
        }
    };

    assert_eq!(value, 30);
}
