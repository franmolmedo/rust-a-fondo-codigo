use std::num::ParseIntError;

fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .split_whitespace()
        .map(str::parse::<i32>)
        .collect()
}

fn main() {
    assert_eq!(parse_numbers("10 20 -3"), Ok(vec![10, 20, -3]));
    assert!(parse_numbers("10 x 20").is_err());
}
