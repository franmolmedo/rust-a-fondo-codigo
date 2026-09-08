use std::cell::RefCell;
use std::collections::HashMap;

let cache = RefCell::new(HashMap::<String, u64>::new());

let value = match cache.borrow().get("clave") {
    Some(value) => *value,
    None => {
        // La guarda de `borrow()` sigue viva durante todo el match:
        cache.borrow_mut().insert(String::from("clave"), 42); // panic
        42
    }
};
