let cached = cache.borrow().get("clave").copied(); // el guard muere aquí

let value = match cached {
    Some(value) => value,
    None => {
        cache.borrow_mut().insert(String::from("clave"), 42);
        42
    }
};
