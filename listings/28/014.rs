use proptest::prelude::*;

fn encode_numbers(values: &[i32]) -> String {
    values.iter().map(i32::to_string).collect::<Vec<_>>().join(",")
}

fn decode_numbers(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    input.split(',').map(str::parse).collect()
}

proptest::proptest! {
    #[test]
    fn number_encoding_round_trips(values in proptest::collection::vec(any::<i32>(), 0..100)) {
        let decoded = decode_numbers(&encode_numbers(&values)).unwrap();
        prop_assert_eq!(decoded, values);
    }
}
