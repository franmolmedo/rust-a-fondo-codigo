use application::order_total;
use domain::{Order, OrderId};
use test_support::FakeOrderRepository;

#[test]
fn a_query_uses_the_contract_with_a_fake() {
    let repository = FakeOrderRepository::new(Some(Order {
        id: OrderId(7),
        total_cents: 1500,
    }));
    assert_eq!(order_total(&repository, OrderId(7)), Some(1500));
    assert_eq!(order_total(&repository, OrderId(8)), None);
}

#[test]
fn a_missing_order_is_not_a_zero_total() {
    let repository = FakeOrderRepository::new(None);
    assert_eq!(order_total(&repository, OrderId(7)), None);
}
