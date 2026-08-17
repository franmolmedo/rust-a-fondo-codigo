use std::ffi::c_char;

unsafe extern "C" {
    pub unsafe fn strlen(text: *const c_char) -> usize;
}
