struct Report {
    title: String,
    values: Vec<u64>,
}

fn assert_send_sync<T: Send + Sync>() {}

assert_send_sync::<Report>();
