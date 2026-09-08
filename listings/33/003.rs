let request = load_user(id); // The body of load_user has not started.
let user = request.await?;
