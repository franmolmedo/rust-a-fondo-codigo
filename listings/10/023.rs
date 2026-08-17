#[derive(Debug, PartialEq)]
enum QuantityError {
    InvalidNumber,
    Zero,
}

fn parse_quantity(input: &str) -> Result<u32, QuantityError> {
    input
        .parse::<u32>()
        .map_err(|_| QuantityError::InvalidNumber)
        .and_then(|value| {
            if value == 0 {
                Err(QuantityError::Zero)
            } else {
                Ok(value)
            }
        })
}

fn main() {
    assert_eq!(parse_quantity("3"), Ok(3));
    assert_eq!(parse_quantity("0"), Err(QuantityError::Zero));
}
