unsafe fn raw_sum(pointer: *const i32, length: usize) -> i32 {
    // SAFETY: the caller supplies one readable, initialized region.
    let values = unsafe { std::slice::from_raw_parts(pointer, length) };
    values.iter().sum()
}

fn safe_sum(values: &[i32]) -> i32 {
    // SAFETY: pointer and length come from this same live slice.
    unsafe { raw_sum(values.as_ptr(), values.len()) }
}

fn main() {
    assert_eq!(safe_sum(&[10, 20, 12]), 42);
    assert_eq!(safe_sum(&[]), 0);
}
