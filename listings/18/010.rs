fn tag<'a, T, const N: usize>(_value: &'a T) -> impl Copy + use<T, N> {
    N
}

fn main() {
    let value = String::from("dato");
    let _tag = tag::<_, 8>(&value);
}
