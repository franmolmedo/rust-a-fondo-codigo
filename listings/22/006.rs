use std::cell::RefCell;
use std::collections::HashMap;

let cache = RefCell::new(HashMap::<String, u64>::new());

let value = match cache.borrow().get("clave") {
    Some(value) => *value,
    None => {
        // El guard de `borrow()` sigue vivo durante todo el match:
        cache.borrow_mut().insert(String::from("clave"), 42); // panic
        42
    }
};
