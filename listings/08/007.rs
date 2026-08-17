enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

fn action(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Draft => "edit",
        OrderStatus::Submitted => "review",
        OrderStatus::Paid => "prepare",
    }
    // error[E0004]: Cancelled is not covered
}

fn main() {}
