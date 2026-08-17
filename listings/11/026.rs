#[derive(Debug)]
struct User {
    name: String,
}

fn names_with_prefix<'a>(
    users: &'a [User],
    prefix: &'a str,
) -> impl Iterator<Item = &'a str> + 'a {
    users
        .iter()
        .map(|user| user.name.as_str())
        .filter(move |name| name.starts_with(prefix))
}

fn main() {
    let users = [
        User { name: String::from("Ada") },
        User { name: String::from("Grace") },
    ];
    assert_eq!(names_with_prefix(&users, "A").collect::<Vec<_>>(), ["Ada"]);
}
