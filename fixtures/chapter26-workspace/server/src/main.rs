use adapters::InMemoryOrderRepository;
use application::order_total;
use domain::{Order, OrderId};

fn main() {
    let mut repository = InMemoryOrderRepository::default();
    repository.insert(Order {
        id: OrderId(1),
        total_cents: 2500,
    });
    let total = order_total(&repository, OrderId(1)).expect("the order was inserted");
    println!("Order total: {total} cents");
}
