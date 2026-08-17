trait Command {
    fn execute(&self) -> &'static str;

    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

struct UnitCommand;

impl Command for UnitCommand {
    fn execute(&self) -> &'static str {
        "ok"
    }

    fn duplicate(&self) -> Self {
        Self
    }
}

fn main() {
    let command: &dyn Command = &UnitCommand;
    assert_eq!(command.execute(), "ok");

    let concrete = UnitCommand;
    let _copy = concrete.duplicate();
}
