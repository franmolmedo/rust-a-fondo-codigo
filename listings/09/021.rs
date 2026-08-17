#[derive(Debug)]
enum UserStatus {
    Active,
    Suspended { reason: String },
}

#[derive(Debug)]
struct User {
    id: u64,
    status: UserStatus,
}

fn suspension(user: &User) -> Option<(u64, &str)> {
    match user {
        User {
            id,
            status: UserStatus::Suspended { reason },
        } => Some((*id, reason)),
        _ => None,
    }
}

fn main() {
    let user = User {
        id: 7,
        status: UserStatus::Suspended {
            reason: String::from("review"),
        },
    };
    assert_eq!(suspension(&user), Some((7, "review")));
}
