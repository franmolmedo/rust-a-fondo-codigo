#[derive(Debug, PartialEq)]
enum PaymentStatus {
    Pending,
    Completed(CompletedPayment),
    Failed(PaymentFailure),
}

#[derive(Debug, PartialEq)]
struct CompletedPayment {
    transaction_id: String,
    amount_cents: u32,
}

#[derive(Debug, PartialEq)]
enum PaymentFailure {
    InsufficientFunds,
    CardExpired,
    FraudSuspected,
    Provider { code: String },
}

fn main() {
    let status = PaymentStatus::Failed(PaymentFailure::Provider {
        code: String::from("P-42"),
    });

    assert!(matches!(
        status,
        PaymentStatus::Failed(PaymentFailure::Provider { ref code }) if code == "P-42"
    ));
}
