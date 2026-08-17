fn convert_all<I, T, U>(input: I) -> Vec<U>
where
    I: IntoIterator<Item = T>,
    U: From<T>,
{
    input.into_iter().map(U::from).collect()
}

fn main() {
    let numbers = convert_all::<_, u16, u64>([10_u16, 20]);
    assert_eq!(numbers, [10_u64, 20]);
}
