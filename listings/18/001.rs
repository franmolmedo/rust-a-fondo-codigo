use std::fmt::Display;

fn label(value: impl Display) -> String {
    format!("valor={value}")
}

fn ids() -> impl Iterator<Item = u64> {
    10..13
}

fn main() {
    assert_eq!(label(42), "valor=42");
    assert_eq!(label("Rust"), "valor=Rust");
    assert_eq!(ids().collect::<Vec<_>>(), [10, 11, 12]);
}
