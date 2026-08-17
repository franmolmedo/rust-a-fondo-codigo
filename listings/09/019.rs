#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn main() {
    let mut message = Message::Text(String::from("hola"));

    match message {
        Message::Text(ref text) => assert_eq!(text, "hola"),
        Message::Quit => {}
    }

    match message {
        Message::Text(ref mut text) => text.push('!'),
        Message::Quit => {}
    }

    assert_eq!(message, Message::Text(String::from("hola!")));
}
