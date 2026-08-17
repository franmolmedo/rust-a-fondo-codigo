unsafe extern "C" {
    // Safe only if the linked symbol really accepts every i32 value this way.
    pub safe fn abs(input: i32) -> i32;

    // The caller must provide a valid NUL-terminated string pointer.
    pub unsafe fn strlen(pointer: *const std::ffi::c_char) -> usize;
}

fn main() {}
