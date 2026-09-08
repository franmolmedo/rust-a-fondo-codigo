let values = [10_u32, 20, 30];
let begin = values.as_ptr();

// SAFETY: two elements are inside the same live array.
let third = unsafe { begin.add(2) };
// SAFETY: `third` points to an initialized, shared-readable `u32`.
assert_eq!(unsafe { third.read() }, 30);

// SAFETY: the one-past pointer may be computed but not read.
let end = unsafe { begin.add(values.len()) };
// SAFETY: advancing one element from `third` reaches the same one-past pointer.
assert_eq!(end, unsafe { third.add(1) });
