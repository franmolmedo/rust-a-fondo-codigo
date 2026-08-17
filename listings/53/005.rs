use std::fmt::Display;

pub fn labels<T: Display>(items: &[T]) -> Vec<String> {
    items.iter().map(ToString::to_string).collect()
}

struct DisplayOnly(u8);

impl Display for DisplayOnly {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "item-{}", self.0)
    }
}

fn main() {
    let values = [DisplayOnly(7)];
    assert_eq!(labels(&values), ["item-7"]);
}
