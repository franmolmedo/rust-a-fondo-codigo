use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static DATA: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);

// Productor
DATA.store(42, Ordering::Relaxed);
READY.store(true, Ordering::Release);
