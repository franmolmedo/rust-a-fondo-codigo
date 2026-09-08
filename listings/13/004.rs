#[derive(Debug)]
struct OrderLine {
    quantity: u32,
    unit_cents: u64,
}

fn total_imperative(lines: &[OrderLine]) -> Option<u64> {
    let mut total = 0_u64;
    for line in lines {
        if line.quantity > 0 {
            let amount = u64::from(line.quantity).checked_mul(line.unit_cents)?;
            total = total.checked_add(amount)?;
        }
    }
    Some(total)
}

fn total_declarative(lines: &[OrderLine]) -> Option<u64> {
    lines
        .iter()
        .filter(|line| line.quantity > 0)
        .map(|line| u64::from(line.quantity).checked_mul(line.unit_cents))
        .try_fold(0_u64, |total, amount| total.checked_add(amount?))
}

fn main() {
    let lines = [
        OrderLine { quantity: 2, unit_cents: 150 },
        OrderLine { quantity: 0, unit_cents: 999 },
    ];
    assert_eq!(total_imperative(&lines), Some(300));
    assert_eq!(total_declarative(&lines), Some(300));
    let too_large = [OrderLine { quantity: 2, unit_cents: u64::MAX }];
    assert_eq!(total_imperative(&too_large), None);
    assert_eq!(total_declarative(&too_large), None);
    let too_many = [
        OrderLine { quantity: 1, unit_cents: u64::MAX },
        OrderLine { quantity: 1, unit_cents: 1 },
    ];
    assert_eq!(total_imperative(&too_many), None);
    assert_eq!(total_declarative(&too_many), None);
}
