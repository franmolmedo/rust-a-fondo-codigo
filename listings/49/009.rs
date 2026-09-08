macro_rules! classify {
    (3) => { "literal three" };
    ($value:expr) => { "opaque expression" };
}

macro_rules! forward {
    ($value:expr) => { classify!($value) };
}

assert_eq!(classify!(3), "literal three");
assert_eq!(forward!(3), "opaque expression");
