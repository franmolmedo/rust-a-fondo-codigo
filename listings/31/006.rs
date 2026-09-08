use std::rc::Rc;

fn require_send<T: Send>(_: T) {}

let shared = Rc::new(5);
let future = async move {
    let value = *shared; // The initial future stores shared.
    std::future::ready(()).await;
    value
};
require_send(future);
// error: future cannot be sent between threads safely
