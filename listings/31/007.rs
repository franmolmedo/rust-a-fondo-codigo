use std::rc::Rc;

fn require_send<T: Send>(_: T) {}

let shared = Rc::new(5);
let value = *shared; // The future will capture only an i32.

let future = async move {
    std::future::ready(()).await;
    value
};
require_send(future);
