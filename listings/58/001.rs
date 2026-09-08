struct User {
    name: String,
}

fn names(users: &[User]) -> impl Iterator<Item = &str> {
    users.iter().map(|user| user.name.as_str())
}

fn main() {
    let users = [User {
        name: String::from("Ferris"),
    }];

    assert_eq!(names(&users).next(), Some("Ferris"));
}
