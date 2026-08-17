use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Pair {
    left: u32,
    right: u32,
}

const _: () = assert!(offset_of!(Pair, left) == 0);
const _: () = assert!(offset_of!(Pair, right) == size_of::<u32>());
const _: () = assert!(size_of::<Pair>() % align_of::<Pair>() == 0);
