#[derive(Debug)]
struct User {
    email: String,
    active: bool,
}

fn main() {
    let mut user = User {
        email: String::from("ada@example.test"),
        active: true,
    };

    user.active = false;
    user.email.push_str(".invalid");

    assert!(!user.active);
}
