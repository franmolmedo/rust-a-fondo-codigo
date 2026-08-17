fn main() {
    let numbers = [10, 20, 30];
    let mut iterator = numbers.iter();

    assert_eq!(iterator.next(), Some(&10));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&30));
    assert_eq!(iterator.next(), None);
}
