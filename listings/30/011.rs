let (commands, worker) = spawn_counter();

commands.send(Command::Add(3)).unwrap();
commands.send(Command::Add(4)).unwrap();

let (reply_tx, reply_rx) = mpsc::channel();
commands.send(Command::Total(reply_tx)).unwrap();
assert_eq!(reply_rx.recv().unwrap(), 7);

drop(commands);                       // Drop the last command sender.
assert_eq!(worker.join().unwrap(), 7); // Wait and read the final total.
