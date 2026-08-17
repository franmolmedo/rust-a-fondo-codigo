use std::sync::Arc;
use std::thread;

struct Config {
    retries: u32,
    endpoint: String,
}

let config = Arc::new(Config {
    retries: 3,
    endpoint: String::from("https://api.example.com"),
});

let handles: Vec<_> = (0..4)
    .map(|worker| {
        let config = Arc::clone(&config);
        thread::spawn(move || {
            format!("worker {worker} usa {} con {} reintentos",
                config.endpoint, config.retries)
        })
    })
    .collect();

for handle in handles {
    println!("{}", handle.join().unwrap());
}
