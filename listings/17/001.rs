trait Codec {
    type Output;
    type Error;
    const NAME: &'static str;

    fn decode(&self, input: &[u8]) -> Result<Self::Output, Self::Error>;
}

struct Utf8;

impl Codec for Utf8 {
    type Output = String;
    type Error = std::str::Utf8Error;
    const NAME: &'static str = "utf-8";

    fn decode(&self, input: &[u8]) -> Result<String, Self::Error> {
        std::str::from_utf8(input).map(str::to_owned)
    }
}

fn main() {
    assert_eq!(Utf8::NAME, "utf-8");
    assert_eq!(Utf8.decode(b"Rust").unwrap(), "Rust");
}
