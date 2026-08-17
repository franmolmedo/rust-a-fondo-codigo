use std::sync::Mutex;

let balance = Mutex::new(100_u64);
{
    let mut value = balance.lock().unwrap();
    *value += 50;
}
