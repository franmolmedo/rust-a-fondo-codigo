#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidNumber,
    Zero,
}

fn parse_non_zero(input: &str) -> Result<u32, ParseError> {
    let value = input
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidNumber)?;

    match value {
        0 => Err(ParseError::Zero),
        value => Ok(value),
    }
}

fn main() {
    assert_eq!(parse_non_zero("7"), Ok(7));
    assert_eq!(parse_non_zero("x"), Err(ParseError::InvalidNumber));
    assert_eq!(parse_non_zero("0"), Err(ParseError::Zero));
}
