struct User {
    name: String,
}

impl User {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let user = User {
        name: String::from("Ada"),
    };

    assert_eq!(user.name(), "Ada");
}
