fn examples() {
    let client = String::from("client");
    let reusable = async move || client.len();
    // `client` belongs to the closure, but each call only borrows it.

    let token = String::from("single-use");
    let consume_once = async move || token;
    // The output moves `token` out: only `AsyncFnOnce` is implemented.

    let _ = (reusable, consume_once);
}
