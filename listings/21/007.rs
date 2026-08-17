use std::rc::Rc;

let config = Rc::new(String::from("local"));
let another_owner = Rc::clone(&config);
assert_eq!(Rc::strong_count(&config), 2);
