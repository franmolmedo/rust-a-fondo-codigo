#[derive(Clone, Copy)]
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
        OrderStatus::Cancelled => "stop",
    }
}

fn main() {
    assert_eq!(action(OrderStatus::Paid), "prepare");
}
