use std::sync::mpsc::{self, Sender};
use std::thread;

enum Command {
    Add(u64),
    Total(Sender<u64>), // Each request includes its own reply channel.
}

fn spawn_counter() -> (Sender<Command>, thread::JoinHandle<u64>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut total = 0_u64; // Only this thread can modify the total.
        for command in rx {
            match command {
                Command::Add(value) => total = total.saturating_add(value),
                Command::Total(reply) => {
                    // Keep running if the caller no longer wants the reply.
                    let _ = reply.send(total);
                }
            }
        }
        total // All senders are gone and the queue is empty.
    });

    (tx, handle)
}
