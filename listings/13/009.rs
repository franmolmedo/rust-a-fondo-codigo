use std::num::ParseIntError;

fn parse_optional_port(raw: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    raw.map(str::parse::<u16>).transpose()
}

fn main() {
    // La frontera real haría: std::env::var("PORT").ok()
    let raw = Some(String::from("8080"));
    let parsed = parse_optional_port(raw.as_deref());
    assert_eq!(parsed, Ok(Some(8080)));
}
