#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    email: Email,
}

#[derive(Debug, PartialEq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Email(String);

fn main() {
    let user = User {
        id: UserId(7),
        email: Email(String::from("ada@example.test")),
    };

    assert_eq!(user.id, UserId(7));
}
