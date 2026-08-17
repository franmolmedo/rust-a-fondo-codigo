fn main() {
    let mut values = vec![1, 2, 3];
    println!("{}", values[0]);
    values.push(4);
    assert_eq!(values, [1, 2, 3, 4]);
}
