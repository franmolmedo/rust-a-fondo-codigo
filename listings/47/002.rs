// SAFETY: la biblioteca publica una única definición de este símbolo y su
// header declara exactamente `uint32_t course_abi_version(void)`.
#[unsafe(no_mangle)]
pub extern "C" fn course_abi_version() -> u32 {
    1
}

assert_eq!(course_abi_version(), 1);
