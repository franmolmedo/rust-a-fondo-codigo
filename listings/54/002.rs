#[derive(Clone, Debug, Eq, PartialEq)]
struct Email(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Role {
    Member,
    Administrator,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct User {
    email: Email,
    role: Role,
}

impl Email {
    fn parse(value: String) -> Result<Self, String> {
        value
            .contains('@')
            .then_some(Self(value))
            .ok_or_else(|| "invalid email".to_owned())
    }
}

fn main() {
    let user = User {
        email: Email::parse("ada@example.test".into()).unwrap(),
        role: Role::Administrator,
    };
    assert_eq!(user.role, Role::Administrator);
}
