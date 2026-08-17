#[derive(Debug, Clone, Copy)]
struct Cents(u64);

#[derive(Debug)]
struct PaymentError;

trait PaymentProcessor {
    fn charge(&self, amount: Cents) -> Result<u64, PaymentError>;
    fn refund(&self, payment_id: u64) -> Result<(), PaymentError>;
}

fn apply_discount<F>(amount: Cents, discount: F) -> Cents
where
    F: FnOnce(Cents) -> Cents,
{
    discount(amount)
}

fn main() {
    let reduced = apply_discount(Cents(1_000), |amount| Cents(amount.0 - 100));
    assert_eq!(reduced.0, 900);
}
