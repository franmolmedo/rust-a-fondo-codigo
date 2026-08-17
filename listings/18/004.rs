fn first() -> impl Iterator<Item = u8> {
    0_u8..3
}

fn second() -> impl Iterator<Item = u8> {
    0_u8..3
}

fn require_same<T>(_left: T, _right: T) {}

fn main() {
    require_same(first(), second());
}
