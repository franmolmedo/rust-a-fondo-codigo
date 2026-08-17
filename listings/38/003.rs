let (user, orders) = tokio::try_join!(
    load_user(id),
    load_orders(id),
)?;
