#[derive(Debug, PartialEq)]
struct User {
    id: u64,
    name: String,
}

fn find_name(users: &[User], id: u64) -> Option<&str> {
    users
        .iter()
        .find(|user| user.id == id)
        .map(|user| user.name.as_str())
}

fn main() {
    let users = vec![User {
        id: 7,
        name: String::from("Ada"),
    }];

    assert_eq!(find_name(&users, 7), Some("Ada"));
    assert_eq!(find_name(&users, 8), None);
}
