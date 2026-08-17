#[derive(Debug, PartialEq)]
struct Header {
    name: String,
    value: String,
}

impl Header {
    fn from_view(view: HeaderView<'_>) -> Self {
        Self {
            name: view.name.to_owned(),
            value: view.value.to_owned(),
        }
    }
}

#[derive(Debug)]
struct HeaderView<'a> {
    name: &'a str,
    value: &'a str,
}

fn main() {
    let view = HeaderView {
        name: "Accept",
        value: "application/json",
    };
    let header = Header::from_view(view);

    assert_eq!(header.name, "Accept");
}
