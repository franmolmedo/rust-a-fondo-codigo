#[derive(Debug, PartialEq)]
struct EmailTooLong {
    maximum: usize,
    actual: usize,
}

impl EmailTooLong {
    fn excess(&self) -> usize {
        self.actual.saturating_sub(self.maximum)
    }
}

#[derive(Debug, PartialEq)]
enum EmailError {
    Empty,
    MissingAt,
    TooLong(EmailTooLong),
}

fn main() {
    let detail = EmailTooLong {
        maximum: 254,
        actual: 260,
    };
    assert_eq!(detail.excess(), 6);
    assert!(matches!(EmailError::TooLong(detail), EmailError::TooLong(_)));
}
