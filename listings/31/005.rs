fn require_send<T: Send>(_: T) {}

let text = String::from("owned");
let future = async move { text.len() };
require_send(future);
