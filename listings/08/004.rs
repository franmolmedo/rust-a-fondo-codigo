#[derive(Debug, PartialEq)]
enum Message {
    Quit,
    Text(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: -2 },
        Message::ChangeColor(20, 40, 60),
    ];

    assert!(matches!(messages[1], Message::Move { x: 3, y: -2 }));
}
