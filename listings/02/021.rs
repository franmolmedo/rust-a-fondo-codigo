struct User {
    active: bool,
}

fn main() {
    let mut user = User { active: false };
    user.active = true;
    assert!(user.active);
}
