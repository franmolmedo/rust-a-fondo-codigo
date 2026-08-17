#[derive(Debug)]
enum Status {
    Draft,
    Ready,
}

#[derive(Debug)]
struct Order {
    status: Status,
    total_cents: u64,
}

fn is_billable(order: &Order) -> bool {
    matches!(order.status, Status::Ready) && order.total_cents > 0
}

fn billable_total(orders: &[Order]) -> u64 {
    orders
        .iter()
        .filter(|order| is_billable(order))
        .map(|order| order.total_cents)
        .sum()
}

fn main() {
    let orders = [
        Order { status: Status::Ready, total_cents: 500 },
        Order { status: Status::Draft, total_cents: 900 },
    ];
    assert_eq!(billable_total(&orders), 500);
}
