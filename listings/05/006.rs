fn main() {
    let mut count = 1;
    let reference = &mut count;

    *reference += 1;
    assert_eq!(*reference, 2);
}
