fn values() -> impl Iterator<Item = u8> {
    0_u8..4
}

fn main() {
    let _ = values().len();
}
