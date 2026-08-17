enum Message {
    Text(String),
    Quit,
}

fn emphasize(message: &mut Message) {
    if let Message::Text(text) = message {
        text.push('!');
    }
}

fn main() {
    let mut message = Message::Text(String::from("hola"));
    emphasize(&mut message);

    match message {
        Message::Text(text) => assert_eq!(text, "hola!"),
        Message::Quit => panic!("se esperaba texto"),
    }
}
