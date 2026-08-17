use std::pin::pin;

let future = pin!(async { 1 });
let inner = future.get_mut();
// error[E0277]: `{async block}` cannot be unpinned
