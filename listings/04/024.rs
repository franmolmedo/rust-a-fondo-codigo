#[derive(Debug)]
struct DraftUser {
    name: String,
}

#[derive(Debug, PartialEq)]
struct RegisteredUser {
    name: String,
}

fn is_valid(user: &DraftUser) -> bool {
    !user.name.trim().is_empty()
}

fn normalize(user: &mut DraftUser) {
    user.name = user.name.trim().to_owned();
}

fn register(user: DraftUser) -> RegisteredUser {
    RegisteredUser { name: user.name }
}

fn main() {
    let mut draft = DraftUser {
        name: String::from("  Ada  "),
    };

    normalize(&mut draft);
    assert!(is_valid(&draft));

    let registered = register(draft);
    assert_eq!(
        registered,
        RegisteredUser {
            name: String::from("Ada"),
        }
    );
}
