use proptest::prelude::*;

proptest::proptest! {
    #[test]
    fn parse_never_panics(input in any::<String>()) {
        let _ = CountryCode::parse(&input);
    }
}
