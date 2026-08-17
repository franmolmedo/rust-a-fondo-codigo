use std::ops::Add;

#[inline(never)]
fn twice<T>(value: T) -> T
where
    T: Copy + Add<Output = T>,
{
    value + value
}

assert_eq!(twice(21_u64), 42);
assert_eq!(twice(1.5_f64), 3.0);
