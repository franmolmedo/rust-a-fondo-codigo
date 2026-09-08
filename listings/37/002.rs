use std::pin::pin;

let future = operation();
let mut future = pin!(future);

// A low-level API can receive future.as_mut().
