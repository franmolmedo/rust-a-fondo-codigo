#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

#[derive(Debug, PartialEq)]
struct Order {
    status: OrderStatus,
}

fn main() {
    let order = Order {
        status: OrderStatus::Submitted,
    };
    assert_eq!(order.status, OrderStatus::Submitted);
}
