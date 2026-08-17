struct Order {
    customer_id: CustomerId, // no Rc<Customer>
    lines: Vec<OrderLine>,
}
