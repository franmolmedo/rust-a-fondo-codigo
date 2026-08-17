async fn update(shared: &std::sync::Mutex<State>) {
    let mut guard = shared.lock().unwrap();
    guard.prepare();
    send_notification().await;
    guard.finish();
}
