// Both operations belong to the current task.
let (a, b) = tokio::join!(load_user(id), load_permissions(id));

// A separate task with its own handle and Send + 'static requirements.
let handle = tokio::spawn(refresh_cache(id));
