#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, PartialEq, Eq)]
struct Email(String);

#[derive(Debug, PartialEq, Eq)]
struct Username(String);

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: UserId,
    email: Email,
    username: Username,
}

fn main() {
    let user = User {
        id: UserId(7),
        email: Email(String::from("ada@example.test")),
        username: Username(String::from("ada")),
    };

    assert_eq!(user.id, UserId(7));
}
