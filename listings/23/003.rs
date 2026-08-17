let snapshot = {
    let state = shared.lock().unwrap();
    state.clone_for_report()
};

write_report(snapshot)?;
