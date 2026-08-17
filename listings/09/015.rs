#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

fn main() {
    let user = User {
        id: 7,
        name: String::from("Ada"),
        active: true,
    };
    let User { id, active, .. } = &user;

    let tuple = (1, 2, 3, 4);
    let (first, .., last) = tuple;

    assert_eq!((*id, *active), (7, true));
    assert_eq!((first, last), (1, 4));
    assert_eq!(user.name, "Ada");
}
