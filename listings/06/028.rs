async fn echo(text: &str) -> &str {
    text
}

fn main() {
    let text = String::from("hola");
    let future = echo(&text);

    drop(future);
    assert_eq!(text, "hola");
}
