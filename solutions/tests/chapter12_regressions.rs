use course_solutions::functional::c12;

#[test]
fn c12_doubling_preserves_large_integer_results() {
    let result = c12::doubled_evens(&[i32::MIN, i32::MAX - 1, 3]);
    assert_eq!(result[0], i64::from(i32::MIN) * 2);
    assert_eq!(result[1], i64::from(i32::MAX - 1) * 2);
    assert_eq!(result.len(), 2);
}

#[test]
fn c12_lazy_mapping_handles_integer_extremes() {
    let (before, after, result) = c12::lazy_map_observation(&[i32::MIN, i32::MAX]);
    assert_eq!((before, after), (0, 2));
    assert_eq!(result[0], i64::from(i32::MIN) * 2);
    assert_eq!(result[1], i64::from(i32::MAX) * 2);
}
