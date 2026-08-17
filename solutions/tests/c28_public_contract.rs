use course_solutions::organization::c28::Percentage;

#[test]
fn percentage_invariant_is_visible_through_the_public_api() {
    assert_eq!(Percentage::new(75).map(Percentage::get), Some(75));
    assert_eq!(Percentage::new(101), None);
}
