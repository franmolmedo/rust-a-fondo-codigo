use std::fmt::Display;

trait Label {
    fn label(&self) -> String;
}

impl<T: Display> Label for T {
    fn label(&self) -> String { self.to_string() }
}

impl Label for u32 {
    fn label(&self) -> String { format!("number:{self}") }
}

fn main() {}
