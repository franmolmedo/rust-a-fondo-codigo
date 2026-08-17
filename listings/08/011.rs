#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    name: String,
}

fn find_user(users: &[User], id: UserId) -> Option<&User> {
    users.iter().find(|user| user.id == id)
}

fn main() {
    let users = vec![User {
        id: UserId(7),
        name: String::from("Ada"),
    }];

    assert_eq!(find_user(&users, UserId(7)).map(|user| user.name.as_str()), Some("Ada"));
    assert_eq!(find_user(&users, UserId(9)), None);
}
