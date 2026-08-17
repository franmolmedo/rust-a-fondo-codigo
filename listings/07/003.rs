#[derive(Debug, PartialEq)]
struct User {
    email: String,
    username: String,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}

fn main() {
    let user = build_user(
        String::from("ada@example.test"),
        String::from("ada"),
    );

    assert_eq!(user.username, "ada");
    assert!(user.active);
}
