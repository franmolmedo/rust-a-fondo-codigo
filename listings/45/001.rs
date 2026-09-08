let mut value = 7_i32;
let read: *const i32 = &raw const value;
let write: *mut i32 = &raw mut value;

// SAFETY: both pointers come from the same live, aligned `value`; the
// operations are sequenced and no references are active.
unsafe {
    assert_eq!(read.read(), 7);
    write.write(8);
}

assert_eq!(value, 8);
