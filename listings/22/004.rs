use std::cell::RefCell;

let names = RefCell::new(vec![String::from("Ada")]);

let reader = names.borrow();
let writer = names.borrow_mut(); // panic: already borrowed: BorrowMutError
println!("{}", reader.len());
