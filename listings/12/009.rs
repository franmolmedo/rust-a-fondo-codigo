fn parse_valid_numbers(inputs: &[&str]) -> Vec<u32> {
    inputs
        .iter()
        .filter_map(|input| input.parse::<u32>().ok())
        .collect()
}

fn main() {
    assert_eq!(parse_valid_numbers(&["10", "x", "20"]), [10, 20]);
}
