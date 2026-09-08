macro_rules! choose {
    ($value:expr) => { missing_function($value) };
    ($($tokens:tt)*) => { 0 };
}

fn main() {
    let _ = choose!(1); // The first arm matches; the second cannot rescue the error.
}
