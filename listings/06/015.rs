struct UserView<'a> {
    name: &'a str,
}

impl<'a> UserView<'a> {
    fn name(&self) -> &'a str {
        self.name
    }

    fn choose_label<'b>(&self, fallback: &'b str) -> &'a str {
        if self.name.is_empty() {
            "anonymous"
        } else {
            let _ = fallback;
            self.name
        }
    }
}

fn main() {
    let name = String::from("Ada");
    let view = UserView { name: &name };

    assert_eq!(view.name(), "Ada");
    assert_eq!(view.choose_label("fallback"), "Ada");
}
