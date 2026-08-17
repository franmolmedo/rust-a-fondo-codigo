//! Tipos para construir pedidos válidos.

/// Identificador opaco de pedido.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(u64);
