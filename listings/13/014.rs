#[derive(Debug)]
struct User {
    name: String,
}

fn short_names_owned(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|user| user.name.len() < 8)
        .map(|user| user.name.clone())
        .collect()
}

fn main() {
    let users = [
        User { name: String::from("Ada") },
        User { name: String::from("Alexandria") },
    ];
    assert_eq!(short_names_owned(&users), [String::from("Ada")]);
}
