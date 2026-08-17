// SAFETY: this artifact defines this symbol exactly once with the published C ABI.
#[unsafe(no_mangle)]
pub extern "C" fn rust_course_library_version() -> u32 {
    1
}

fn main() {
    assert_eq!(rust_course_library_version(), 1);
}
