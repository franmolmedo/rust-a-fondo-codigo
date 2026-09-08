async fn bad() {
    loop {
        perform_cpu_step(); // Never yields to the async executor.
    }
}
