/// Creates a shared slice tied to a lifetime chosen by the caller.
///
/// # Safety
///
/// `pointer` and `length` must satisfy every `slice::from_raw_parts`
/// precondition, and the memory must remain valid and immutable for `'a`.
unsafe fn view<'a, T>(pointer: *const T, length: usize) -> &'a [T] {
    // SAFETY: delegated verbatim to this function's caller contract.
    unsafe { std::slice::from_raw_parts(pointer, length) }
}

fn main() {}
