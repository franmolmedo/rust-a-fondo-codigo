fn forged<'a>() -> &'a i32 {
    let local = 42;
    // El cast no liga `'a` a `local`: la referencia escapará de su storage.
    unsafe { &*(&raw const local) }
}

fn main() {
    let _dangling = forged();
}
