#[derive(Debug, PartialEq)]
enum PaymentMethod {
    Cash,
    Card,
    BankTransfer,
}

fn main() {
    let method = PaymentMethod::Card;
    assert_eq!(method, PaymentMethod::Card);
}
