use std::rc::Rc;

struct CachedReport {
    title: String,
    render_cache: Rc<String>, // este campo elimina Send y Sync
}

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<CachedReport>();
    // error[E0277]: `Rc<String>` cannot be sent between threads safely
    // note: required because it appears within the type `CachedReport`
}
