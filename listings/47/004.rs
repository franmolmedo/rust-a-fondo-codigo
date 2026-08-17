use std::ffi::{CStr, c_char};

unsafe fn raw_strlen(pointer: *const c_char) -> usize {
    // SAFETY: esta función privada delega las premisas a su caller.
    unsafe { CStr::from_ptr(pointer) }.to_bytes().len()
}

fn c_length(text: &CStr) -> usize {
    // SAFETY: `text` aporta región viva, non-null y terminada en NUL; la
    // función simulada no conserva el puntero tras retornar.
    unsafe { raw_strlen(text.as_ptr()) }
}

assert_eq!(c_length(c"Rust"), 4);
