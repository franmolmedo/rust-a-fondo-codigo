#[derive(Clone, Copy)]
enum OrderStatus {
    Draft,
    Submitted,
    Paid,
    Cancelled,
}

impl OrderStatus {
    fn can_cancel(self) -> bool {
        matches!(self, Self::Draft | Self::Submitted)
    }

    fn is_terminal(self) -> bool {
        matches!(self, Self::Paid | Self::Cancelled)
    }
}

fn main() {
    assert!(OrderStatus::Draft.can_cancel());
    assert!(!OrderStatus::Paid.can_cancel());
    assert!(OrderStatus::Cancelled.is_terminal());
}
