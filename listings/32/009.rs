// Consumidor
while !READY.load(Ordering::Acquire) {
    std::hint::spin_loop();
}
assert_eq!(DATA.load(Ordering::Relaxed), 42); // garantizado tras observar READY
