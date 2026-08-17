use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

let shutdown = Arc::new(AtomicBool::new(false));
let flag = Arc::clone(&shutdown);

let worker = thread::spawn(move || {
    let mut processed = 0_u64;
    while !flag.load(Ordering::Relaxed) {
        processed += 1; // una unidad de trabajo
    }
    processed
});

shutdown.store(true, Ordering::Relaxed);
let total = worker.join().unwrap();
println!("procesadas {total} unidades");
