use std::cell::Cell;

fn replace<'a>(slot: &Cell<&'a str>, value: &'a str) {
    slot.set(value);
}

fn main() {
    let first = String::from("A");
    let second = String::from("B");
    let slot = Cell::new(first.as_str());
    replace(&slot, &second);
    assert_eq!(slot.get(), "B");
}
