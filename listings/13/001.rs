#[derive(Debug)]
struct User {
    email: String,
    active: bool,
}

fn active_emails(users: &[User]) -> Vec<&str> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.email.as_str())
        .collect()
}

fn main() {
    let users = [
        User { email: String::from("ada@example.com"), active: true },
        User { email: String::from("grace@example.com"), active: false },
    ];
    assert_eq!(active_emails(&users), ["ada@example.com"]);
}
