let user_future = load_user(id);
let permissions_future = load_permissions(id);
let (user, permissions) = tokio::join!(user_future, permissions_future);
