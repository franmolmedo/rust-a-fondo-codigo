//! The consumer defines the repository interface it needs.

use domain::{Order, OrderId};

pub trait OrderRepository {
    fn find(&self, id: OrderId) -> Option<Order>;
}

pub fn order_total(repository: &impl OrderRepository, id: OrderId) -> Option<u64> {
    repository.find(id).map(|order| order.total_cents)
}
