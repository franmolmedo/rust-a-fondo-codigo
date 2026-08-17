#[derive(Clone, Copy)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

fn is_waiting(status: OrderStatus) -> bool {
    match status {
        OrderStatus::Pending => true,
        _ => false,
    }
}

fn main() {
    assert!(is_waiting(OrderStatus::Pending));
    assert!(!is_waiting(OrderStatus::Shipped));
}
