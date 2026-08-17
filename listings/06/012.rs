struct UserView<'a> {
    name: &'a str,
}

fn local_view<'a>() -> UserView<'a> {
    let name = String::from("Ada");
    UserView { name: &name }
    // error[E0515]: cannot return a value referencing local data
}

fn main() {}
