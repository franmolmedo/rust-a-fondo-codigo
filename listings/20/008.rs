fn accepts_any(text: &str) -> usize {
    text.len()
}

fn narrow_inputs<'short, 'long: 'short>(
    callback: fn(&'short str) -> usize,
) -> fn(&'long str) -> usize {
    callback
}

fn main() {
    let only_receives_static: fn(&'static str) -> usize = accepts_any;
    assert_eq!(only_receives_static("Rust"), 4);
    let narrower = narrow_inputs(accepts_any);
    assert_eq!(narrower("Cargo"), 5);
}
