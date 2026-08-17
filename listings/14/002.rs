type UserIdAlias = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrderId(u64);

fn user_path(id: UserId) -> String {
    format!("/users/{}", id.0)
}

fn main() {
    let alias: UserIdAlias = 7_u64;
    let raw: u64 = alias; // Es exactamente el mismo tipo.
    assert_eq!(raw, 7);
    assert_eq!(user_path(UserId(7)), "/users/7");
    let _order = OrderId(7);
}
