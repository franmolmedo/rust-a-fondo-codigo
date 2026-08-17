fn normalized_port(input: Option<&str>) -> Option<u16> {
    input
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .and_then(|text| text.parse::<u16>().ok())
        .filter(|port| *port != 0)
}

fn main() {
    assert_eq!(normalized_port(Some(" 8080 ")), Some(8080));
    assert_eq!(normalized_port(Some("0")), None);
    assert_eq!(normalized_port(None), None);
}
