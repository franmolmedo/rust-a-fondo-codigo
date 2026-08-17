fn identity<T>(value: T) -> T {
    value
}

fn main() {
    assert_eq!(identity(10), 10);
    assert_eq!(identity(String::from("hola")), "hola");
}
