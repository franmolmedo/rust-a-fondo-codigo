use std::sync::mpsc;
use std::thread;
use std::time::Duration;

let (tx, rx) = mpsc::sync_channel::<u32>(2);

let producer = thread::spawn(move || {
    for value in 0..5 {
        // Con 2 elementos en vuelo, este send BLOQUEA hasta que
        // el consumidor retire uno: el productor no puede adelantarse.
        tx.send(value).unwrap();
    }
});

for value in rx {
    thread::sleep(Duration::from_millis(10)); // simula consumidor lento
    println!("procesando {value}");
}

producer.join().unwrap();
