#[derive(Debug, PartialEq)]
enum Event {
    Connected { user_id: u64 },
    Disconnected { user_id: u64, reason: String },
}

fn user_id(event: &Event) -> u64 {
    match event {
        Event::Connected { user_id }
        | Event::Disconnected { user_id, .. } => *user_id,
    }
}

fn main() {
    let event = Event::Disconnected {
        user_id: 7,
        reason: String::from("timeout"),
    };

    assert_eq!(user_id(&event), 7);
}
