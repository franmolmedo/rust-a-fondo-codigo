trait Convert<Input> {
    type Output;
    fn convert(&self, input: Input) -> Self::Output;
}

struct Length;

impl Convert<String> for Length {
    type Output = usize;
    fn convert(&self, input: String) -> usize { input.len() }
}

impl Convert<&str> for Length {
    type Output = usize;
    fn convert(&self, input: &str) -> usize { input.len() }
}

fn main() {
    assert_eq!(Length.convert(String::from("Rust")), 4);
    assert_eq!(Length.convert("GAT"), 3);
}
