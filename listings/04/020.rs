struct Ticket {
    id: String,
}

impl Drop for Ticket {
    fn drop(&mut self) {}
}

fn main() {
    let mut ticket = Ticket {
        id: String::from("T-42"),
    };

    let id = std::mem::take(&mut ticket.id);
    assert_eq!(id, "T-42");
    assert!(ticket.id.is_empty());
}
