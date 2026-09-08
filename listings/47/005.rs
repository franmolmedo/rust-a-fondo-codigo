use std::ffi::c_int;

const OK: c_int = 0;
const NULL_OUTPUT: c_int = -1;
const NULL_INPUT: c_int = -2;
const OVERFLOW: c_int = -3;

/// # Safety
///
/// Null pointers are handled as documented error or empty-input cases.
/// A non-null `output` must be aligned, writable as a `u64`, and not accessed
/// elsewhere during this call. If both pointers are non-null and `len > 0`,
/// `values` must be aligned and readable for `len` initialized `u32` elements
/// in one live allocation, disjoint from `output` and not modified during
/// this call. The byte size must fit in `isize::MAX`, without address wrap.
// SAFETY: this example owns the unique symbol `rust_a_fondo_sum_u32_v1`.
#[unsafe(export_name = "rust_a_fondo_sum_u32_v1")]
pub unsafe extern "C" fn sum_u32(
    values: *const u32,
    len: usize,
    output: *mut u64,
) -> c_int {
    if output.is_null() {
        return NULL_OUTPUT;
    }
    let values = if len == 0 {
        &[]
    } else {
        if values.is_null() {
            return NULL_INPUT;
        }
        // SAFETY: the public contract provides range, initialization,
        // lifetime, and aliasing guarantees.
        unsafe { std::slice::from_raw_parts(values, len) }
    };
    let Some(sum) = values
        .iter()
        .try_fold(0_u64, |sum, value| sum.checked_add(u64::from(*value)))
    else {
        return OVERFLOW;
    };
    // SAFETY: `output` is writable, and the input has already been consumed.
    unsafe { output.write(sum) };
    OK
}

let values = [10_u32, 20, 12];
let mut output = 0_u64;
// SAFETY: input and output are live, aligned, disjoint regions.
assert_eq!(unsafe { sum_u32(values.as_ptr(), 3, &mut output) }, OK);
assert_eq!(output, 42);
