macro_rules! classify {
    (3) => { "literal tres" };
    ($value:expr) => { "expresión opaca" };
}

macro_rules! forward {
    ($value:expr) => { classify!($value) };
}

assert_eq!(classify!(3), "literal tres");
assert_eq!(forward!(3), "expresión opaca");
