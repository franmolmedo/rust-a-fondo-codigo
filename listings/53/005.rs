use std::fmt::Display;
use std::rc::Rc;

pub fn labels<T: Display>(items: &[T]) -> Vec<String> {
    items.iter().map(ToString::to_string).collect()
}

struct DisplayOnly<'a> {
    label: &'a str,
    _not_send_or_sync: Rc<()>,
}

impl Display for DisplayOnly<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.label)
    }
}

fn main() {
    let label = String::from("item-7");
    let values = [DisplayOnly {
        label: &label,
        _not_send_or_sync: Rc::new(()),
    }];
    assert_eq!(labels(&values), ["item-7"]);
}
