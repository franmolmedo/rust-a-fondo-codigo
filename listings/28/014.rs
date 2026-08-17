use proptest::prelude::*;

proptest::proptest! {
    #[test]
    fn reversing_twice_restores_input(values in proptest::collection::vec(any::<i32>(), 0..100)) {
        let reversed_twice: Vec<_> = values.iter().rev().rev().copied().collect();
        prop_assert_eq!(reversed_twice, values);
    }
}
