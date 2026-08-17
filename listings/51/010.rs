#![deny(unsafe_op_in_unsafe_fn)]

unsafe fn read_copy<T: Copy>(pointer: *const T) -> T {
    // SAFETY: el caller garantiza que pointer está alineado, es legible y
    // apunta a un T inicializado durante toda esta lectura.
    unsafe { pointer.read() }
}

let value = 21_u32;
// SAFETY: &value produce un puntero válido, alineado e inicializado.
assert_eq!(unsafe { read_copy(&value) }, 21);
