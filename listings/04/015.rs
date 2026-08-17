fn main() {
    let first = String::from("hola");
    let mut second = first.clone();
    second.push('!');

    assert_eq!(first, "hola");
    assert_eq!(second, "hola!");
}
