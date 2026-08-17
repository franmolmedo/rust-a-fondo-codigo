// crates/application/src/lib.rs
use domain::{Order, OrderId};

#[derive(Debug)]
pub struct RepositoryError(pub String);

pub trait OrderRepository {
    fn find(&self, id: OrderId) -> Result<Option<Order>, RepositoryError>;
    fn save(&self, order: &Order) -> Result<(), RepositoryError>;
}

pub struct PlaceOrder<R> {
    repository: R,
}

impl<R: OrderRepository> PlaceOrder<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, order: Order) -> Result<(), RepositoryError> {
        // reglas del caso de uso...
        self.repository.save(&order)
    }
}
