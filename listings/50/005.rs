trait Marker {}

struct Wrapper<'a, T, const N: usize>
where
    T: 'a,
{
    values: &'a [T; N],
}

impl<'a, T, const N: usize> Marker for Wrapper<'a, T, N>
where
    T: 'a,
{}

let values = [String::from("Rust")];
let wrapper = Wrapper { values: &values };
assert_eq!(wrapper.values.len(), 1);
