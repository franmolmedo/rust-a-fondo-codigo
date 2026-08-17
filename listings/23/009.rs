use std::sync::{Condvar, Mutex};

struct Queue {
    items: Mutex<Vec<u64>>,
    ready: Condvar,
}

impl Queue {
    fn push(&self, item: u64) {
        self.items.lock().unwrap().push(item);
        self.ready.notify_one();
    }

    fn pop_blocking(&self) -> u64 {
        let mut items = self.items.lock().unwrap();
        loop {
            if let Some(item) = items.pop() {
                return item;
            }
            items = self.ready.wait(items).unwrap();
        }
    }
}
