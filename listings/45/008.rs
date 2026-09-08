use std::ptr;

fn swap_disjoint<T>(values: &mut [T], left: usize, right: usize) -> bool {
    if left >= values.len() || right >= values.len() || left == right {
        return false;
    }

    let base = values.as_mut_ptr();
    // SAFETY: both indices belong to the same slice and are distinct; the
    // `&mut [T]` borrow prevents outside access during this operation.
    unsafe { ptr::swap(base.add(left), base.add(right)) };
    true
}

let mut values = ["a", "b", "c"];
assert!(swap_disjoint(&mut values, 0, 2));
assert_eq!(values, ["c", "b", "a"]);
