#[derive(Debug, PartialEq)]
enum Message {
    Text(String),
    Quit,
}

fn text_length(message: &Message) -> Option<usize> {
    match message {
        Message::Text(text) => Some(text.len()),
        Message::Quit => None,
    }
}

fn main() {
    let message = Message::Text(String::from("hola"));

    assert_eq!(text_length(&message), Some(4));
    assert_eq!(message, Message::Text(String::from("hola")));
}
