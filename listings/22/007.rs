use std::cell::RefCell;
use std::collections::HashMap;

let cache = RefCell::new(HashMap::<String, u64>::new());

let value = if let Some(value) = cache.borrow().get("clave") {
    *value
} else {
    // Edición 2024: el guard ya se liberó al entrar aquí.
    cache.borrow_mut().insert(String::from("clave"), 42);
    42
};

assert_eq!(value, 42);
