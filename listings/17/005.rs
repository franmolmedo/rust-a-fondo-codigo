trait Parse {
    type Output;
    fn parse(&self, input: &str) -> Option<Self::Output>;
}

struct Flexible;

impl Parse for Flexible {
    type Output = u32;
    fn parse(&self, input: &str) -> Option<u32> { input.parse().ok() }
}

impl Parse for Flexible {
    type Output = f64;
    fn parse(&self, input: &str) -> Option<f64> { input.parse().ok() }
}

fn main() {}
