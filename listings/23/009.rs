use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

struct Queue {
    items: Mutex<VecDeque<u64>>,
    ready: Condvar,
}

impl Queue {
    fn push(&self, item: u64) {
        self.items.lock().unwrap().push_back(item);
        self.ready.notify_one();
    }

    fn pop_blocking(&self) -> u64 {
        let mut items = self.items.lock().unwrap();
        loop {
            if let Some(item) = items.pop_front() {
                return item;
            }
            items = self.ready.wait(items).unwrap();
        }
    }
}

let queue = Queue { items: Mutex::new(VecDeque::new()), ready: Condvar::new() };
queue.push(10);
queue.push(20);
assert_eq!(queue.pop_blocking(), 10);
assert_eq!(queue.pop_blocking(), 20);
