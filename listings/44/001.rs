fn main() {
    let pointer: *const i32 = std::ptr::null();

    // This compiles, but running it would attempt an invalid access: UB.
    let _value = unsafe { *pointer };
}
