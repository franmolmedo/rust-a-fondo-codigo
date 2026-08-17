let user_future = load_user(id);
let permissions_future = load_permissions(id);
let (user, permissions) = join(user_future, permissions_future).await;
