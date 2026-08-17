fn main() {
    let mut count = 0;
    let mut increment = || count += 1;

    increment();
    increment();

    assert_eq!(count, 2);
}
