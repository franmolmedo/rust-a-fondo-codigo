use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
let worker = thread::spawn(move || {
    tx.send(String::from("done")).unwrap();
});

let message = rx.recv().unwrap();
worker.join().unwrap();
assert_eq!(message, "done");
