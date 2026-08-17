fn make_name() -> String {
    String::from("Grace")
}

fn identity(text: String) -> String {
    text
}

fn main() {
    let name = make_name();
    let same_name = identity(name);
    assert_eq!(same_name, "Grace");
}
