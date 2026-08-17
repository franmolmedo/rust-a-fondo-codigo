use std::cell::Cell;

let message = Cell::new(String::from("pendiente"));

let previous = message.replace(String::from("enviado"));
assert_eq!(previous, "pendiente");

let last = message.take(); // deja String::default() dentro
assert_eq!(last, "enviado");
assert_eq!(message.take(), "");
