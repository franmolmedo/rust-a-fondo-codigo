#[doc(hidden)]
pub fn __normalize(input: &str) -> String {
    input.trim().to_owned()
}

#[macro_export]
macro_rules! normalized {
    ($input:expr) => {
        $crate::__normalize($input)
    };
}

fn main() {
    assert_eq!(normalized!(" Rust "), "Rust");
}
