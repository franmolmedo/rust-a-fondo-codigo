#[derive(Debug, PartialEq)]
struct Order {
    total_cents: u64,
}

trait OrderIteratorExt: Iterator<Item = Order> + Sized {
    fn billable(self) -> impl Iterator<Item = Order> {
        self.filter(|order| order.total_cents > 0)
    }
}

impl<I> OrderIteratorExt for I where I: Iterator<Item = Order> {}

fn main() {
    let total: u64 = vec![
        Order { total_cents: 0 },
        Order { total_cents: 500 },
    ]
    .into_iter()
    .billable()
    .map(|order| order.total_cents)
    .sum();

    assert_eq!(total, 500);
}
