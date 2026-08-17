trait ParseAs<Output> {
    fn parse_as(&self, input: &str) -> Option<Output>;
}

struct Flexible;

impl ParseAs<u32> for Flexible {
    fn parse_as(&self, input: &str) -> Option<u32> {
        input.parse().ok()
    }
}

impl ParseAs<f64> for Flexible {
    fn parse_as(&self, input: &str) -> Option<f64> {
        input.parse().ok()
    }
}

fn main() {
    let parser = Flexible;
    let integer: Option<u32> = ParseAs::parse_as(&parser, "42");
    let decimal: Option<f64> = ParseAs::parse_as(&parser, "4.2");
    assert_eq!(integer, Some(42));
    assert_eq!(decimal, Some(4.2));
}
