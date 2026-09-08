use course_solutions::organization::c25::{
    AuditedService, Email, InvalidEmail, Order, OrderId, StableReport, UserId, parse_port,
};

#[test]
fn c25_external_consumers_use_the_facade_without_private_module_paths() {
    let order = Order::new(OrderId(7));
    assert_eq!(order.id(), OrderId(7));
    assert_eq!(UserId(3).0, 3);
    assert_eq!(StableReport(5).0, 5);
    assert_eq!(AuditedService::new("  ORDERS  ").name(), "orders");
}

#[test]
fn c25_private_fields_are_exposed_through_checked_construction_and_accessors() {
    assert_eq!(Email::parse("invalid"), Err(InvalidEmail));
    assert_eq!(
        Email::parse("ada@example.test").unwrap().as_str(),
        "ada@example.test"
    );
    assert_eq!(parse_port("65535").unwrap(), u16::MAX);
    assert!(parse_port("65536").is_err());
}
