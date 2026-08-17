let mut value = 7_i32;
let read: *const i32 = &raw const value;
let write: *mut i32 = &raw mut value;

// SAFETY: ambos punteros proceden del mismo `value`, que sigue vivo y
// alineado; las operaciones están secuenciadas y no hay referencias activas.
unsafe {
    assert_eq!(read.read(), 7);
    write.write(8);
}

assert_eq!(value, 8);
