#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let first = User {
        email: String::from("ada@example.test"),
        username: String::from("ada"),
        active: true,
    };

    let second = User {
        email: String::from("grace@example.test"),
        ..first
    };

    assert_eq!(second.username, "ada");
    assert_eq!(first.email, "ada@example.test");
    assert!(first.active);
}
