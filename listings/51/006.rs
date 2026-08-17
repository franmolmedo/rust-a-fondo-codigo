use std::cell::RefCell;

fn stored_values() -> usize {
    let values = RefCell::new(vec![10, 20, 30]);
    values.borrow().len()
}

assert_eq!(stored_values(), 3);
