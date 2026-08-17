#[derive(Debug, PartialEq)]
struct User {
    name: String,
    active: bool,
}

fn active_names(users: &[User]) -> Vec<&str> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.as_str())
        .collect()
}

fn active_names_owned(users: &[User]) -> Vec<String> {
    users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.name.clone())
        .collect()
}

fn main() {
    let users = [
        User { name: String::from("Ada"), active: true },
        User { name: String::from("Grace"), active: false },
    ];
    assert_eq!(active_names(&users), ["Ada"]);
    assert_eq!(active_names_owned(&users), [String::from("Ada")]);
}
