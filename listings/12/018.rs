#[derive(Debug, PartialEq)]
enum SumError {
    InvalidNumber { index: usize },
    Overflow { index: usize },
}

fn parse_and_sum(inputs: &[&str]) -> Result<u32, SumError> {
    inputs.iter().enumerate().try_fold(0_u32, |total, (index, input)| {
        let value = input
            .parse::<u32>()
            .map_err(|_| SumError::InvalidNumber { index })?;
        total
            .checked_add(value)
            .ok_or(SumError::Overflow { index })
    })
}

fn main() {
    assert_eq!(parse_and_sum(&["10", "20"]), Ok(30));
    assert_eq!(
        parse_and_sum(&["10", "x"]),
        Err(SumError::InvalidNumber { index: 1 })
    );
}
