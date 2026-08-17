fn wait_forever() -> ! {
    loop {
        std::hint::spin_loop();
    }
}
