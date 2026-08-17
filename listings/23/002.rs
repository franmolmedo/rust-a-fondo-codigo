use std::{sync::{Arc, Mutex}, thread};

let counter = Arc::new(Mutex::new(0_u64));
let handles: Vec<_> = (0..4)
    .map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || *counter.lock().unwrap() += 1)
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}

assert_eq!(*counter.lock().unwrap(), 4);
