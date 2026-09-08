use course_solutions::unsafe_low_level::c44::{
    FfiSliceError, checked_get, ffi_sum, replace_after_successful_build, safe_sum,
};
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

#[test]
fn c44_integer_overflow_is_reported_in_both_sum_interfaces() {
    for values in [[i32::MAX, 1], [i32::MIN, -1]] {
        assert_eq!(safe_sum(&values), None);
        // SAFETY: both arguments describe this live, shared array for the call.
        assert_eq!(
            unsafe { ffi_sum(values.as_ptr(), values.len()) },
            Err(FfiSliceError::Overflow)
        );
    }
}

#[test]
fn c44_build_failure_does_not_undo_interior_mutation() {
    let mut value = Cell::new(1);
    let failed = catch_unwind(AssertUnwindSafe(|| {
        replace_after_successful_build(&mut value, |old| {
            old.set(2);
            panic!("construction failed");
        });
    }));
    assert!(failed.is_err());
    assert_eq!(value.get(), 2);
}

#[test]
fn c44_maximum_index_and_zero_sized_elements_stay_in_bounds() {
    assert_eq!(checked_get(&[()], 0), Some(&()));
    assert_eq!(checked_get(&[()], usize::MAX), None);
    let empty = [0_i32; 0];
    // SAFETY: the zero-length array supplies a non-null, aligned pointer.
    assert_eq!(unsafe { ffi_sum(empty.as_ptr(), 0) }, Ok(0));
}
