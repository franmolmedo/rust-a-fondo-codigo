use proptest::prelude::*;

fn parse_key_value(input: &str) -> Result<(&str, &str), &'static str> {
    let (key, value) = input.split_once('=').ok_or("missing =")?;
    if key.is_empty() {
        return Err("empty key");
    }
    Ok((key, value))
}

proptest::proptest! {
    #![proptest_config(ProptestConfig::with_cases(1_000))]
    #[test]
    fn parse_never_panics(input in any::<String>()) {
        let _ = parse_key_value(&input);
    }
}
