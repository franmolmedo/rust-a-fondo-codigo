/// Reserva stock para todas las líneas del pedido.
///
/// La reserva es atómica: o se reservan todas las líneas o ninguna.
///
/// # Examples
///
/// ```
/// # use catalog::{Inventory, Order};
/// let mut inventory = Inventory::with_stock("SKU-1", 10);
/// let order = Order::single("SKU-1", 3);
/// inventory.reserve(&order)?;
/// assert_eq!(inventory.available("SKU-1"), 7);
/// # Ok::<(), catalog::ReserveError>(())
/// ```
///
/// # Errors
///
/// Devuelve [`ReserveError::Insufficient`] si alguna línea supera el stock
/// disponible. En ese caso ninguna línea queda reservada.
pub fn reserve(&mut self, order: &Order) -> Result<(), ReserveError> {
    /* ... */
}
