use course_solutions::unsafe_low_level::c45::{
    LocalCounter, MoveError, TaggedRef, copy_within_raw, swap_disjoint,
};

#[test]
fn c45_counter_stops_at_the_limit_without_changing_its_value() {
    let counter = LocalCounter::new(u64::MAX - 1);
    assert_eq!(counter.increment(), Some(u64::MAX));
    assert_eq!(counter.increment(), None);
    assert_eq!(counter.get(), u64::MAX);
}

#[test]
fn c45_empty_and_zero_sized_ranges_are_valid_but_overflow_is_not() {
    let mut empty: [u32; 0] = [];
    assert_eq!(copy_within_raw(&mut empty, 0..0, 0), Ok(()));
    let mut values = [1, 2];
    assert_eq!(copy_within_raw(&mut values, 2..2, 2), Ok(()));
    assert_eq!(
        copy_within_raw(&mut values, 0..1, usize::MAX),
        Err(MoveError::InvalidDestination)
    );
    assert_eq!(values, [1, 2]);
    let mut units = [(); 3];
    assert_eq!(copy_within_raw(&mut units, 0..2, 1), Ok(()));
    assert_eq!(swap_disjoint(&mut units, 0, 2), Ok(()));
}

#[test]
fn c45_tag_bits_do_not_change_the_recovered_reference() {
    let owner = 42_u64;
    for tag in [false, true] {
        let reference = TaggedRef::new(&owner, tag).unwrap();
        assert_eq!(reference.tag(), tag);
        assert!(std::ptr::eq(reference.get(), &owner));
    }
}
