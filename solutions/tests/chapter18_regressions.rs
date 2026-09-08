use course_solutions::abstraction::c18::{minimum_length, positive_doubled};

#[test]
fn c18_positive_doubled_accepts_the_full_i32_range() {
    let values = [i32::MIN, -1, 0, 1, i32::MAX];
    assert_eq!(
        positive_doubled(&values).collect::<Vec<_>>(),
        [2, i64::from(i32::MAX) * 2]
    );
    assert_eq!(positive_doubled(&[]).next(), None);
}

#[test]
fn c18_minimum_length_is_measured_in_bytes() {
    let at_least_two_bytes = minimum_length(2);
    assert!(at_least_two_bytes("ñ"));
    assert!(!at_least_two_bytes("n"));
    assert!(minimum_length(0)(""));
}
