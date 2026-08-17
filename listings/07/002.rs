#[derive(Debug, PartialEq)]
struct User {
    id: UserId,
    active: bool,
}

#[derive(Debug, PartialEq)]
struct UserId(u64);

#[derive(Debug, PartialEq)]
struct Production;

fn main() {
    let user = User {
        id: UserId(42),
        active: true,
    };
    let environment = Production;

    assert_eq!(user.id, UserId(42));
    assert!(user.active);
    assert_eq!(format!("{environment:?}"), "Production");
}
