/// # Safety
/// The pair must satisfy every `slice::from_raw_parts` precondition for this call.
unsafe fn raw_sum(pointer: *const i32, length: usize) -> Option<i32> {
    // SAFETY: guaranteed by the caller contract above.
    let values = unsafe { std::slice::from_raw_parts(pointer, length) };
    values.iter().try_fold(0_i32, |sum, value| sum.checked_add(*value))
}

fn safe_sum(values: &[i32]) -> Option<i32> {
    // SAFETY: pointer and length come from this same live slice.
    unsafe { raw_sum(values.as_ptr(), values.len()) }
}

fn main() {
    assert_eq!(safe_sum(&[10, 20, 12]), Some(42));
    assert_eq!(safe_sum(&[]), Some(0));
    assert_eq!(safe_sum(&[i32::MAX, 1]), None);
}
