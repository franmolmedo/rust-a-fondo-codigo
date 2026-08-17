async fn echo(text: &str) -> &str {
    text
}

fn require_static<T: 'static>(_value: T) {}

fn main() {
    let text = String::from("hola");
    let future = echo(&text);

    require_static(future);
    // error[E0597]: text would need to be borrowed for 'static
}
