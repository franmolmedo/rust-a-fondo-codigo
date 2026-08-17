async fn bad() {
    loop {
        perform_cpu_step(); // nunca cede
    }
}
