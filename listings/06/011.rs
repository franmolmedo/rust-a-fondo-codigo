#[derive(Debug, PartialEq)]
struct UserView<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Ada");
    let view = UserView { name: &name };

    assert_eq!(view, UserView { name: "Ada" });
}
