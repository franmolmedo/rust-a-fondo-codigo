use std::ffi::c_int;

const OK: c_int = 0;
const NULL_OUTPUT: c_int = -1;
const NULL_INPUT: c_int = -2;
const OVERFLOW: c_int = -3;

/// # Safety
///
/// `output` debe ser escribible como `u64`. Si `len > 0`, `values` describe
/// `len` elementos inicializados dentro de una allocation viva y disjunta.
unsafe extern "C" fn sum_u32(
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
        // SAFETY: el contrato público aporta rango, init, vida y aliasing.
        unsafe { std::slice::from_raw_parts(values, len) }
    };
    let Some(sum) = values
        .iter()
        .try_fold(0_u64, |sum, value| sum.checked_add(u64::from(*value)))
    else {
        return OVERFLOW;
    };
    // SAFETY: `output` es escribible y el input ya se consumió.
    unsafe { output.write(sum) };
    OK
}

let values = [10_u32, 20, 12];
let mut output = 0_u64;
// SAFETY: input y output son regiones vivas, alineadas y disjuntas.
assert_eq!(unsafe { sum_u32(values.as_ptr(), 3, &mut output) }, OK);
assert_eq!(output, 42);
