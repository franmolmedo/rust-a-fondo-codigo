#[derive(Debug, PartialEq)]
struct Command {
    name: String,
    retries: u8,
}

struct User {
    name: String,
}

fn parse(input: &str) -> Command {
    Command {
        name: input.trim().to_owned(),
        retries: 0,
    }
}

fn enrich(command: &mut Command) {
    command.retries = 3;
}

fn enriched(mut command: Command) -> Command {
    command.retries = 3;
    command
}

fn user_name(user: &User) -> &str {
    &user.name
}

fn main() {
    let source = String::from(" build ");
    let mut command = parse(&source);
    enrich(&mut command);
    let command = enriched(command);

    let user = User {
        name: String::from("Ada"),
    };

    assert_eq!(source, " build ");
    assert_eq!(command, Command {
        name: String::from("build"),
        retries: 3,
    });
    assert_eq!(user_name(&user), "Ada");
}
