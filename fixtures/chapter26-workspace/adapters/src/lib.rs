//! An in-memory adapter; a database adapter would live in this layer too.

use application::OrderRepository;
use domain::{Order, OrderId};
use std::collections::HashMap;

#[derive(Default)]
pub struct InMemoryOrderRepository {
    orders: HashMap<OrderId, Order>,
}

impl InMemoryOrderRepository {
    pub fn insert(&mut self, order: Order) {
        self.orders.insert(order.id, order);
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn find(&self, id: OrderId) -> Option<Order> {
        self.orders.get(&id).cloned()
    }
}
