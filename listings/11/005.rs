fn main() {
    let factor = 10;
    let multiply = |value| value * factor;

    assert_eq!(multiply(3), 30);
    assert_eq!(factor, 10);
}
