fn bad() -> &'static str {
    let text = String::from("hola");
    &text
    // error[E0515]: cannot return a reference to local data
}

fn main() {}
