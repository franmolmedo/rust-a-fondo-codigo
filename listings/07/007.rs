#[derive(Debug, PartialEq)]
struct User {
    email: String,
}

#[derive(Debug, PartialEq)]
struct UserView<'a> {
    email: &'a str,
}

fn view(user: &User) -> UserView<'_> {
    UserView { email: &user.email }
}

fn main() {
    let user = User {
        email: String::from("ada@example.test"),
    };

    assert_eq!(view(&user).email, "ada@example.test");
}
