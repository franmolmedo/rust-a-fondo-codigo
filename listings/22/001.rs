use std::cell::Cell;

struct Metrics {
    hits: Cell<u64>,
}

impl Metrics {
    fn record(&self) {
        self.hits.set(self.hits.get() + 1);
    }
}
