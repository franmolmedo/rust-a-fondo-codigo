struct User {
    active: bool,
}

fn first_active(users: &[User]) -> Option<&User> {
    users.iter().find(|user| user.active)
}
