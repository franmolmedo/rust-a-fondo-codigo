#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn emphasize(message: &mut Message) -> Option<usize> {
    match message {
        Message::Text(text) => {
            text.push('!');
            Some(text.len())
        }
        Message::Quit => None,
    }
}

fn main() {
    let mut message = Message::Text(String::from("hola"));

    if let Message::Text(text) = &message {
        assert_eq!(text, "hola");
    }

    assert_eq!(emphasize(&mut message), Some(5));
    assert_eq!(message, Message::Text(String::from("hola!")));
}
