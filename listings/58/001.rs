fn names(users: &[User]) -> impl Iterator<Item = &str> {
    users.iter().map(|user| user.name.as_str())
}
