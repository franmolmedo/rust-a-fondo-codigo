fn forged<'a>() -> &'a i32 {
    let local = 42;
    // The cast does not tie `'a` to `local`: the reference escapes its storage.
    unsafe { &*(&raw const local) }
}

fn main() {
    let _dangling = forged();
}
