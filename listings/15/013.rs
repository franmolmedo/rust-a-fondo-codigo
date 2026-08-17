use std::rc::Rc;

struct Shared<T> {
    inner: Rc<T>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self { inner: Rc::clone(&self.inner) }
    }
}

struct Connection;

fn main() {
    let shared = Shared { inner: Rc::new(Connection) };
    let copy = shared.clone();
    assert!(Rc::ptr_eq(&shared.inner, &copy.inner));
}
