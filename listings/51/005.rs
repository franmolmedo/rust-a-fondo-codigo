use std::cell::RefCell;

let values = RefCell::new(Vec::<u8>::new());
if let Some(value) = values.borrow_mut().pop() {
    assert!(value > 0);
} else {
    // In Rust 2024, the temporary RefMut from the condition is already gone.
    values.borrow_mut().push(1);
}

assert_eq!(values.into_inner(), vec![1]);
