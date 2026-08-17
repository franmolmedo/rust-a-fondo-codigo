#[derive(Debug, PartialEq)]
struct HeaderView<'a> {
    name: &'a str,
    value: &'a str,
}

fn parse_header(line: &str) -> Option<HeaderView<'_>> {
    let (name, value) = line.split_once(':')?;
    Some(HeaderView {
        name: name.trim(),
        value: value.trim(),
    })
}

fn main() {
    let line = String::from("Content-Type: text/plain");
    let header = parse_header(&line).unwrap();

    assert_eq!(header.name, "Content-Type");
    assert_eq!(header.value, "text/plain");
}
