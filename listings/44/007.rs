unsafe extern "C" {
    // Safe only if the linked symbol really accepts every f64 value this way.
    pub safe fn sqrt(input: f64) -> f64;

    // The caller must provide a valid NUL-terminated string pointer.
    pub unsafe fn strlen(pointer: *const std::ffi::c_char) -> usize;
}

fn main() {}
