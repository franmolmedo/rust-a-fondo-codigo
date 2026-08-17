fn main() {
    let number = 10;
    let reference = &number;

    let &copied = reference;
    assert_eq!(copied, 10);
}
