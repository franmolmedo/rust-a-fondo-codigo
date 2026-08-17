trait Clock {
    fn now(&self) -> u64; // epoch millis, o un tipo Timestamp propio
}

struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}
