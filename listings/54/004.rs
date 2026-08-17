fn prepare_user(email: String, role: Role) -> Result<User, EmailError> {
    Ok(User {
        email: Email::parse(email)?,
        role,
    })
}

fn register(
    store: &mut impl UserStore,
    events: &mut impl EventSink,
    user: User,
) -> Result<(), RegisterError> {
    store.insert_unique(user.clone())?;
    events.user_registered(&user)?;
    Ok(())
}
