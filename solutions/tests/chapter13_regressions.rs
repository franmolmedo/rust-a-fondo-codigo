use course_solutions::functional::c13;

#[test]
fn c13_selected_total_can_exceed_u64() {
    assert_eq!(c13::total_selected(&[u64::MAX]), u128::from(u64::MAX) * 2);
    assert_eq!(c13::total_selected(&[]), 0);
}

#[test]
fn c13_generic_total_stops_at_overflow() {
    let mut values = [u64::MAX, 1, 7].into_iter();
    assert_eq!(c13::total(values.by_ref()), None);
    assert_eq!(values.next(), Some(7));
    assert_eq!(c13::total([]), Some(0));
}

#[test]
fn c13_review_preserves_large_totals_and_rejections() {
    let report = c13::review(&[(1, u64::MAX), (2, 0), (3, 1)]);
    assert_eq!(report.total_cents, u128::from(u64::MAX) + 1);
    assert_eq!(report.accepted, [1, 3]);
    assert_eq!(report.rejected, [2]);
}
