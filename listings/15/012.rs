use std::rc::Rc;

#[derive(Clone)]
struct Shared<T> {
    inner: Rc<T>,
}

struct Connection;

fn main() {
    let shared = Shared { inner: Rc::new(Connection) };
    let _copy = shared.clone();
    // Connection no implementa Clone, aunque Rc<Connection> sí.
}
