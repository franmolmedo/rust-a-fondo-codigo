fn create_user(
    email: String,
    admin: bool,
    users: &mut Vec<(String, bool)>,
    events: &mut Vec<String>,
) -> Result<(), String> {
    if !email.contains('@') {
        return Err("bad email".to_owned());
    }
    if users.iter().any(|(existing, _)| existing == &email) {
        return Err("duplicate".to_owned());
    }
    users.push((email.clone(), admin));
    events.push(format!("created:{email}"));
    Ok(())
}

fn main() {
    let mut users = Vec::new();
    let mut events = Vec::new();
    create_user("ada@example.test".into(), true, &mut users, &mut events).unwrap();
    assert_eq!(users, [("ada@example.test".to_owned(), true)]);
    assert_eq!(events, ["created:ada@example.test"]);
}
