#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cents(u64);

#[derive(Debug, PartialEq, Eq)]
struct DraftInvoice {
    lines: Vec<Cents>,
}

#[derive(Debug, PartialEq, Eq)]
struct IssuedInvoice {
    number: u64,
    lines: Vec<Cents>,
    total: Cents,
}

#[derive(Debug, PartialEq, Eq)]
enum IssueError {
    Empty,
    Overflow,
}

impl DraftInvoice {
    fn issue(self, number: u64) -> Result<IssuedInvoice, (Self, IssueError)> {
        if self.lines.is_empty() {
            return Err((self, IssueError::Empty));
        }
        let total = self.lines.iter().try_fold(0_u64, |sum, line| {
            sum.checked_add(line.0).ok_or(IssueError::Overflow)
        });
        match total {
            Ok(total) => Ok(IssuedInvoice {
                number,
                lines: self.lines,
                total: Cents(total),
            }),
            Err(error) => Err((self, error)),
        }
    }
}

fn main() {
    let draft = DraftInvoice { lines: vec![Cents(200), Cents(300)] };
    let issued = draft.issue(41).unwrap();
    assert_eq!(issued.number, 41);
    assert_eq!(issued.lines.len(), 2);
    assert_eq!(issued.total, Cents(500));
}
