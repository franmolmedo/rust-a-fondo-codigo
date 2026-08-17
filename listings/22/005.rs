let first_borrow = names.borrow();
println!("{}", first_borrow.len());
drop(first_borrow);

names.borrow_mut().clear();
