#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cents(u64);

#[derive(Debug, PartialEq, Eq)]
enum Invoice {
    Draft { lines: Vec<Cents> },
    Issued { number: u64, lines: Vec<Cents>, total: Cents },
}

#[derive(Debug, PartialEq, Eq)]
enum IssueError {
    Empty,
    Overflow,
    AlreadyIssued,
}

impl Invoice {
    fn issue(&mut self, number: u64) -> Result<(), IssueError> {
        match self {
            Self::Draft { lines } if lines.is_empty() => Err(IssueError::Empty),
            Self::Draft { lines } => {
                let total = lines
                    .iter()
                    .try_fold(0_u64, |sum, line| sum.checked_add(line.0))
                    .map(Cents)
                    .ok_or(IssueError::Overflow)?;
                let lines = std::mem::take(lines);
                *self = Self::Issued { number, lines, total };
                Ok(())
            }
            Self::Issued { .. } => Err(IssueError::AlreadyIssued),
        }
    }
}

fn main() {
    let mut invoice = Invoice::Draft { lines: vec![Cents(500)] };
    assert_eq!(invoice.issue(7), Ok(()));
    assert_eq!(invoice.issue(8), Err(IssueError::AlreadyIssued));
    match invoice {
        Invoice::Issued { number, lines, total } => {
            assert_eq!((number, lines.len(), total), (7, 1, Cents(500)));
        }
        Invoice::Draft { .. } => panic!("la transición debía completarse"),
    }

    let mut overflowing = Invoice::Draft {
        lines: vec![Cents(u64::MAX), Cents(1)],
    };
    assert_eq!(overflowing.issue(9), Err(IssueError::Overflow));
    assert!(matches!(overflowing, Invoice::Draft { .. }));
}
