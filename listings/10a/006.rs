#[derive(Debug, PartialEq)]
enum QuantityError {
    Zero,
    AboveMaximum { maximum: u32, actual: u32 },
}

fn quantity(value: u32) -> Result<u32, QuantityError> {
    match value {
        0 => Err(QuantityError::Zero),
        1..=100 => Ok(value),
        actual => Err(QuantityError::AboveMaximum {
            maximum: 100,
            actual,
        }),
    }
}

fn main() {
    assert_eq!(
        quantity(120),
        Err(QuantityError::AboveMaximum {
            maximum: 100,
            actual: 120,
        })
    );
}
