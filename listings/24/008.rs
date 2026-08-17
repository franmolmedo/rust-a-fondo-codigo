use std::pin::pin;

let future = async { 42 };
let mut pinned = pin!(future);
