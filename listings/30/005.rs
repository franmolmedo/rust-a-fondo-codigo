use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
let worker = thread::spawn(move || {
    tx.send(String::from("terminado")).unwrap();
});

let message = rx.recv().unwrap();
worker.join().unwrap();
assert_eq!(message, "terminado");
