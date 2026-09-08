fn main() {
    let mut remaining = 3;
    while remaining > 0 {
        remaining -= 1;
    }
    assert_eq!(remaining, 0);

    let mut total = 0;
    for number in 1..=5 {
        if number == 3 {
            continue;
        }
        total += number;
    }
    assert_eq!(total, 12);
}
