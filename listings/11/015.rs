use std::thread;

fn main() {
    let values = vec![1, 2, 3];
    let handle = thread::spawn(move || values.into_iter().sum::<i32>());

    assert_eq!(handle.join().unwrap(), 6);
}
