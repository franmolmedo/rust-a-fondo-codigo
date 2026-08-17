/// Suma dos cantidades comprobando overflow.
///
/// ```
/// # use catalog::checked_total;
/// assert_eq!(checked_total([2, 3]), Some(5));
/// ```
pub fn checked_total(values: impl IntoIterator<Item = u64>) -> Option<u64> {
    values.into_iter().try_fold(0, u64::checked_add)
}
