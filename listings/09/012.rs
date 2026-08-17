enum Event {
    Created { id: u64 },
    Idle,
}

fn id(event: Event) -> u64 {
    match event {
        Event::Created { id } | Event::Idle => id,
        // error[E0408]: id is not bound in all patterns
    }
}

fn main() {}
