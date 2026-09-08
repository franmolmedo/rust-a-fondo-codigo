// SAFETY: the library publishes a single definition of this symbol, and its
// header declares exactly `uint32_t course_abi_version(void)`.
#[unsafe(no_mangle)]
pub extern "C" fn course_abi_version() -> u32 {
    1
}

assert_eq!(course_abi_version(), 1);
