// Chapter 38: both operations advance within the same task.
let (user, permissions) = tokio::join!(load_user(id), load_permissions(id));
