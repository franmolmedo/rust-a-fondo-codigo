#[derive(Debug, PartialEq)]
struct ReviewReport {
    accepted: Vec<u64>,
    rejected: Vec<u64>,
    total_cents: u64,
}

fn review(requests: &[(u64, u64)]) -> ReviewReport {
    let mut report = ReviewReport {
        accepted: Vec::new(),
        rejected: Vec::new(),
        total_cents: 0,
    };

    for &(id, amount) in requests {
        if amount == 0 {
            report.rejected.push(id);
            continue;
        }
        report.accepted.push(id);
        report.total_cents += amount;
    }
    report
}

fn main() {
    assert_eq!(
        review(&[(1, 500), (2, 0)]),
        ReviewReport { accepted: vec![1], rejected: vec![2], total_cents: 500 }
    );
}
