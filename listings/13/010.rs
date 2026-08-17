use std::num::ParseIntError;

fn parse_all(inputs: &[&str]) -> Result<Vec<u32>, ParseIntError> {
    inputs
        .iter()
        .map(|input| input.parse::<u32>())
        .collect()
}

fn main() {
    assert_eq!(parse_all(&["10", "20"]), Ok(vec![10, 20]));
    assert!(parse_all(&["10", "bad", "20"]).is_err());
}
