#[derive(Debug)]
struct OrderLine {
    quantity: u32,
    unit_cents: u64,
}

fn total_imperative(lines: &[OrderLine]) -> u64 {
    let mut total = 0;
    for line in lines {
        if line.quantity > 0 {
            total += u64::from(line.quantity) * line.unit_cents;
        }
    }
    total
}

fn total_declarative(lines: &[OrderLine]) -> u64 {
    lines
        .iter()
        .filter(|line| line.quantity > 0)
        .map(|line| u64::from(line.quantity) * line.unit_cents)
        .sum()
}

fn main() {
    let lines = [
        OrderLine { quantity: 2, unit_cents: 150 },
        OrderLine { quantity: 0, unit_cents: 999 },
    ];
    assert_eq!(total_imperative(&lines), 300);
    assert_eq!(total_declarative(&lines), 300);
}
