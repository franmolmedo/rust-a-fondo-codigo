#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn inspect(message: &Message) -> Option<&str> {
    match message {
        Message::Text(text) => Some(text),
        Message::Quit => None,
    }
}

fn modify(message: &mut Message) {
    if let Message::Text(text) = message {
        text.push('!');
    }
}

fn consume(message: Message) -> Option<String> {
    match message {
        Message::Text(text) => Some(text),
        Message::Quit => None,
    }
}

fn main() {
    let mut message = Message::Text(String::from("hola"));
    assert_eq!(inspect(&message), Some("hola"));

    modify(&mut message);
    assert_eq!(consume(message), Some(String::from("hola!")));
}
