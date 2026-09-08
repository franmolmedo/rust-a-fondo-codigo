// The two operations together are NOT atomic.
let current = counter.load(Ordering::Relaxed);
counter.store(current + 1, Ordering::Relaxed);
