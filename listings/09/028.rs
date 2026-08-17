#[derive(Debug, PartialEq)]
struct Email(String);

#[derive(Debug, PartialEq)]
struct Username(String);

#[derive(Debug, PartialEq)]
enum UserCommand {
    Register {
        email: Email,
        username: Username,
    },
    Suspend {
        user_id: u64,
        reason: String,
    },
    Delete {
        user_id: u64,
    },
}

fn audit(command: &UserCommand) -> String {
    match command {
        UserCommand::Register { email, username } => {
            format!("register {} as {}", email.0, username.0)
        }
        UserCommand::Suspend { user_id, reason } => {
            format!("suspend {user_id}: {reason}")
        }
        UserCommand::Delete { user_id } => format!("delete {user_id}"),
    }
}

fn main() {
    let command = UserCommand::Suspend {
        user_id: 7,
        reason: String::from("review"),
    };

    assert_eq!(audit(&command), "suspend 7: review");
}
