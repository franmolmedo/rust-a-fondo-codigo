trait Lend {
    type Item<'a>
    where
        Self: 'a;

    fn lend<'a>(&'a self) -> Self::Item<'a>;
}

struct Text(String);

impl Lend for Text {
    type Item<'a> = &'a str where Self: 'a;

    fn lend<'a>(&'a self) -> &'a str {
        &self.0
    }
}

fn main() {
    assert_eq!(Text(String::from("Rust")).lend(), "Rust");
}
