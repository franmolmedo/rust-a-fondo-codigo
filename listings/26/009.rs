// crates/adapters/src/http.rs
pub struct CreateOrderRequest {
    pub customer_id: u64,
    pub lines: Vec<LineRequest>,
}

impl TryFrom<CreateOrderRequest> for application::PlaceOrderCommand {
    type Error = ValidationError;

    fn try_from(request: CreateOrderRequest) -> Result<Self, Self::Error> {
        // aquí se valida, se convierten unidades y se rechaza lo inválido
        /* ... */
    }
}
