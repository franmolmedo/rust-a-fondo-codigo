use std::fmt::Display;

fn describe<T>(value: &T) -> String
where
    T: Display + ?Sized,
{
    value.to_string()
}

fn main() {
    let number = 42;
    let erased: &dyn Display = &number;
    assert_eq!(describe(&number), "42");
    assert_eq!(describe(erased), "42");
}
