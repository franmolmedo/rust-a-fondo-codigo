fn twice<T>(value: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    value + value
}

fn main() {
    assert_eq!(twice(21_i32), 42);       // instancia para i32
    assert_eq!(twice(1.5_f64), 3.0);    // instancia para f64
}
