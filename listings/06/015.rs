struct UserView<'a> {
    name: &'a str,
}

impl<'a> UserView<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn choose_label<'b>(&'b self, fallback: &'b str) -> &'b str {
        if self.name.is_empty() {
            fallback
        } else {
            self.name
        }
    }
}

fn main() {
    let name = String::from("Ada");
    let view = UserView { name: &name };

    assert_eq!(view.name(), "Ada");
    assert_eq!(view.choose_label("fallback"), "Ada");

    let empty = UserView { name: "" };
    let fallback = String::from("Guest");
    assert_eq!(empty.choose_label(&fallback), "Guest");
}
