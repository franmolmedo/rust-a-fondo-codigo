let ticket = {
    let mut state = shared.lock().unwrap();
    state.prepare_notification()
};

send_notification(&ticket.payload).await;

let mut state = shared.lock().unwrap();
state.finish_if_revision_matches(ticket.revision);
