fn shorten<'long: 'short, 'short>(value: &'long str) -> &'short str {
    value
}

fn main() {
    let text = String::from("válido");
    let view = shorten(&text);
    assert_eq!(view, "válido");
}
