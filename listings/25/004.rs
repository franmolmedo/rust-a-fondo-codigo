// src/lib.rs
pub mod domain {
    pub mod order {
        pub struct Order {
            pub id: u64,
        }

        pub fn validate(order: &Order) -> bool {
            super::shared_rule(order.id) // sube al módulo `domain`
        }
    }

    fn shared_rule(id: u64) -> bool {
        id != 0
    }
}

pub fn report(order: &domain::order::Order) -> String {
    format!("pedido {}", order.id)
}
