trait ContainerFamily {
    type Container<T, const N: usize>;
}

struct Arrays;

impl ContainerFamily for Arrays {
    type Container<T, const N: usize> = [T; N];
}

fn main() {
    let values: <Arrays as ContainerFamily>::Container<u16, 3> = [10, 20, 30];
    assert_eq!(values, [10, 20, 30]);
}
