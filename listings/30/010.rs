use std::sync::mpsc::{self, Sender};
use std::thread;

enum Command {
    Add(u64),
    Total(Sender<u64>), // el canal de respuesta viaja dentro del comando
}

fn spawn_counter() -> (Sender<Command>, thread::JoinHandle<u64>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut total = 0; // estado SIN locks: un solo dueño
        for command in rx {
            match command {
                Command::Add(value) => total += value,
                Command::Total(reply) => {
                    // Si el solicitante ya no espera, ignorar es la política:
                    let _ = reply.send(total);
                }
            }
        }
        total // el for terminó: no quedan senders vivos
    });

    (tx, handle)
}
