use std::ffi::{CStr, c_char};

/// # Safety
/// `pointer` must satisfy `CStr::from_ptr` for the duration of this call.
unsafe fn raw_strlen(pointer: *const c_char) -> usize {
    // SAFETY: this private function delegates the requirements to its caller.
    unsafe { CStr::from_ptr(pointer) }.to_bytes().len()
}

fn c_length(text: &CStr) -> usize {
    // SAFETY: `text` provides a live, non-null, NUL-terminated region; the
    // simulated function does not retain the pointer after returning.
    unsafe { raw_strlen(text.as_ptr()) }
}

assert_eq!(c_length(c"Rust"), 4);
