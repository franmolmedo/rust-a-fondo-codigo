fn partition_parse(inputs: &[&str]) -> (Vec<u32>, Vec<usize>) {
    let mut values = Vec::new();
    let mut invalid_indices = Vec::new();

    for (index, input) in inputs.iter().enumerate() {
        match input.parse::<u32>() {
            Ok(value) => values.push(value),
            Err(_) => invalid_indices.push(index),
        }
    }

    (values, invalid_indices)
}

fn main() {
    assert_eq!(partition_parse(&["10", "x", "20"]), (vec![10, 20], vec![1]));
}
