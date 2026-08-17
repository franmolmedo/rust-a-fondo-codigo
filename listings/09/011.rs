#[derive(Debug)]
enum Event {
    Created { id: u64 },
    Updated { id: u64 },
    Deleted { id: u64 },
}

fn changed_id(event: Event) -> u64 {
    match event {
        Event::Created { id }
        | Event::Updated { id }
        | Event::Deleted { id } => id,
    }
}

fn main() {
    assert_eq!(changed_id(Event::Updated { id: 7 }), 7);
}
