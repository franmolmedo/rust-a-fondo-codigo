fn greeting() -> String {
    String::from("hola")
}

fn main() {
    let text = greeting();
    assert_eq!(text, "hola");
}
