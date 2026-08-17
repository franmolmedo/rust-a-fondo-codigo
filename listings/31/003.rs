use std::{cell::RefCell, sync::Arc};

let state = Arc::new(RefCell::new(0));
std::thread::spawn(move || *state.borrow_mut() += 1);
