use std::pin::pin;

let future = operation();
let mut future = pin!(future);

// Una API de bajo nivel puede recibir future.as_mut().
