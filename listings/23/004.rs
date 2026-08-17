use std::sync::Mutex;

#[derive(Default)]
struct Counts {
    total: u32,
    reserved: u32,
}

struct Inventory {
    counts: Mutex<Counts>,
}

impl Inventory {
    fn reserve(&self, amount: u32) -> bool {
        let mut counts = self.counts.lock().unwrap();
        let Some(after) = counts.reserved.checked_add(amount) else {
            return false;
        };
        if after > counts.total {
            return false;
        }
        counts.reserved = after;
        true
    }

    fn snapshot(&self) -> (u32, u32) {
        let counts = self.counts.lock().unwrap();
        (counts.total, counts.reserved)
    }
}

let inventory = Inventory {
    counts: Mutex::new(Counts { total: 10, reserved: 0 }),
};
assert!(inventory.reserve(4));
assert_eq!(inventory.snapshot(), (10, 4));
