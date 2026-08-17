fn parse_non_zero(input: &str) -> Option<u16> {
    input.parse::<u16>().ok().filter(|port| *port != 0)
}

fn main() {
    let raw = Some("8080");
    let nested = raw.map(parse_non_zero);
    let flat = raw.and_then(parse_non_zero);

    assert_eq!(nested, Some(Some(8080)));
    assert_eq!(flat, Some(8080));
}
