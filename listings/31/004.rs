use std::sync::{Arc, Mutex};

let state = Arc::new(Mutex::new(0));
let worker_state = Arc::clone(&state);

std::thread::spawn(move || *worker_state.lock().unwrap() += 1)
    .join()
    .unwrap();

assert_eq!(*state.lock().unwrap(), 1);
