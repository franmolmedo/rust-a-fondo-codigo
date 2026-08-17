#[derive(Debug, PartialEq)]
enum Command<'a> {
    Help,
    Create { name: &'a str },
    Delete { id: &'a str },
}

fn parse<'a>(args: &'a [&'a str]) -> Result<Command<'a>, &'static str> {
    match args {
        ["help"] => Ok(Command::Help),
        ["user", "create", name] => Ok(Command::Create { name }),
        ["user", "delete", id] => Ok(Command::Delete { id }),
        _ => Err("uso inválido"),
    }
}

fn main() {
    assert_eq!(
        parse(&["user", "create", "ada"]),
        Ok(Command::Create { name: "ada" })
    );
}
