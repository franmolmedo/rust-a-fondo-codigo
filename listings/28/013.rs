trait Clock {
    fn now(&self) -> u64; // milisegundos desde la época Unix, o un tipo Timestamp propio
}

struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}
