fn words() -> impl Iterator<Item = &'static str> {
    let text = String::from("hello rust");
    text.split_whitespace()
    // error[E0515]: devuelve un valor que referencia text
}

fn main() {}
