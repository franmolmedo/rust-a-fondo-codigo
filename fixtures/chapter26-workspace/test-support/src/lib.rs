//! A fake for consumers of the application, not a dependency of it.

use application::OrderRepository;
use domain::{Order, OrderId};

pub struct FakeOrderRepository {
    order: Option<Order>,
}

impl FakeOrderRepository {
    pub fn new(order: Option<Order>) -> Self {
        Self { order }
    }
}

impl OrderRepository for FakeOrderRepository {
    fn find(&self, id: OrderId) -> Option<Order> {
        self.order.as_ref().filter(|order| order.id == id).cloned()
    }
}
