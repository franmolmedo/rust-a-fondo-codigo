struct Ticket {
    label: String,
}

#[inline(never)]
fn inspect_ticket(ticket: Ticket) -> usize {
    ticket.label.len()
}

assert_eq!(inspect_ticket(Ticket { label: String::from("MIR") }), 3);
