fn main() {
    let offset = 1_u32;
    let operation = async move |value: u32| value + offset;
    let _future = operation(41);
}
