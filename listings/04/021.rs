struct Counter {
    value: u32,
}

impl Counter {
    fn value(&self) -> u32 {
        self.value
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn finish(self) -> u32 {
        self.value
    }
}

fn main() {
    let mut counter = Counter { value: 1 };
    assert_eq!(counter.value(), 1);

    counter.increment();
    assert_eq!(counter.value(), 2);

    let final_value = counter.finish();
    assert_eq!(final_value, 2);
}
