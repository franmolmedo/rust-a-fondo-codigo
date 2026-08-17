fn main() {
    let pointer: *const i32 = std::ptr::null();

    // Compila, pero ejecutarlo intentaría crear un acceso inválido: UB.
    let _value = unsafe { *pointer };
}
