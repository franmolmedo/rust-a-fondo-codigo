let (user, orders) = tokio::join!(load_user(id), load_orders(id));
let user = user?;
let orders = orders?;
