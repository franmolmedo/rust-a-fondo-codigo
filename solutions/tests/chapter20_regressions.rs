use course_solutions::abstraction::c20::{Consumes, Owns, apply_identity, identity};
use std::rc::Rc;

#[test]
fn c20_higher_ranked_return_can_also_use_static_data() {
    fn constant(_value: &str) -> &str {
        "fallback"
    }
    let local = String::from("local");
    assert_eq!(apply_identity(identity, &local), "local");
    assert_eq!(apply_identity(constant, &local), "fallback");
}

#[test]
fn c20_function_marker_does_not_inherit_rc_thread_restrictions() {
    fn require_send_sync<T: Send + Sync>(value: T) -> T {
        value
    }
    let consumes = require_send_sync(Consumes::<Rc<()>>::new(7));
    assert_eq!(consumes.id(), 7);
    assert_eq!(require_send_sync(Owns::<String>::new(8)).id(), 8);
}
