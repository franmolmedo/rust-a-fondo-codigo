//! Domain data without storage or transport dependencies.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OrderId(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Order {
    pub id: OrderId,
    pub total_cents: u64,
}
