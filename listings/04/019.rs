struct Ticket {
    id: String,
}

impl Drop for Ticket {
    fn drop(&mut self) {
        println!("cerrando {}", self.id);
    }
}

fn main() {
    let ticket = Ticket {
        id: String::from("T-42"),
    };
    let id = ticket.id;
    println!("{id}");
    // error[E0509]: cannot move out of type `Ticket`, which implements `Drop`
}
