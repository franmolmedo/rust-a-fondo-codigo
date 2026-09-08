// Consumer: observe the publication before reading DATA.
while !READY.load(Ordering::Acquire) {
    std::hint::spin_loop();
}
assert_eq!(DATA.load(Ordering::Relaxed), 42);
