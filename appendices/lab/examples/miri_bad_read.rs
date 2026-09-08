// A negative control: only Miri interprets the invalid read. Native builds
// never execute undefined behavior, even if this example is run accidentally.
#[cfg(miri)]
fn main() {
    let bytes = [1_u8, 2, 3];
    // Intentionally invalid: the allocation has three bytes, but u32 needs four.
    // This is a diagnostic exercise, not an example of a justified unsafe block.
    let value = unsafe { bytes.as_ptr().cast::<u32>().read_unaligned() };
    println!("{value}");
}

#[cfg(not(miri))]
fn main() {
    eprintln!("Run this negative control with cargo miri run; native execution is disabled.");
}
