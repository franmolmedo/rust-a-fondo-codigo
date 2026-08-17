use std::cell::RefCell;

let names = RefCell::new(vec![String::from("Ada")]);
names.borrow_mut().push(String::from("Linus"));
assert_eq!(names.borrow().len(), 2);
