fn shorten<'short>(value: &'static str) -> &'short str {
    value
}

fn pair_with_local<'a>(long: &'static str, local: &'a str) -> (&'a str, &'a str) {
    (long, local)
}

fn main() {
    let local = String::from("corto");
    let (left, right) = pair_with_local("largo", &local);
    assert_eq!((left, right), ("largo", "corto"));
    assert_eq!(shorten("estático"), "estático");
}
