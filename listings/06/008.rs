fn make() -> String {
    String::from("hola")
}

fn main() {
    let text = make();
    assert_eq!(text, "hola");
}
