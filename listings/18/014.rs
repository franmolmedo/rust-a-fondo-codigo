fn jobs() -> impl Iterator<Item = u8> {
    0_u8..3
}

fn assert_send<T: Send>(_value: T) {}

fn main() {
    assert_send(jobs());
}
