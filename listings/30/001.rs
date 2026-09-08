use std::thread;

let handle = thread::spawn(|| {
    (1..=100).sum::<u64>()
});

let total = handle.join().expect("worker panicked");
assert_eq!(total, 5050);
