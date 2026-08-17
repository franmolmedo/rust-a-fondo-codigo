use std::num::ParseIntError;

#[derive(Debug)]
struct ParseAtIndexError {
    index: usize,
    source: ParseIntError,
}

fn parse_with_indices(inputs: &[&str]) -> Result<Vec<u32>, ParseAtIndexError> {
    inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            input
                .parse::<u32>()
                .map_err(|source| ParseAtIndexError { index, source })
        })
        .collect()
}

fn main() {
    let error = parse_with_indices(&["10", "bad", "20"]).unwrap_err();
    assert_eq!(error.index, 1);
    assert!(!error.source.to_string().is_empty());
}
