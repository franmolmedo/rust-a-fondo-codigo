macro_rules! choose {
    ($value:expr) => { missing_function($value) };
    ($($tokens:tt)*) => { 0 };
}

fn main() {
    let _ = choose!(1); // coincide con el primer brazo; el segundo no rescata el error
}
