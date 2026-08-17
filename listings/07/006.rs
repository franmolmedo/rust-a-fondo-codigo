struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let first = User {
        email: String::from("ada@example.test"),
        username: String::from("ada"),
        active: true,
    };
    let _second = User {
        email: String::from("grace@example.test"),
        ..first
    };

    println!("{}", first.username);
    // error[E0382]: username was moved
}
