use std::sync::mpsc;
use std::thread;
use std::time::Duration;

let (tx, rx) = mpsc::sync_channel::<u32>(2);

let producer = thread::spawn(move || {
    for value in 0..5 {
        // Wait for space whenever both buffer slots are occupied.
        tx.send(value).unwrap();
    }
});

for value in rx {
    thread::sleep(Duration::from_millis(10)); // Simulate a slow consumer.
    println!("processing {value}");
}

producer.join().unwrap();
