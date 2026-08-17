trait StrExt {
    fn non_blank(&self) -> Option<&str>;
}

impl StrExt for str {
    fn non_blank(&self) -> Option<&str> {
        let trimmed = self.trim();
        (!trimmed.is_empty()).then_some(trimmed)
    }
}

fn main() {
    assert_eq!("  Rust ".non_blank(), Some("Rust"));
    assert_eq!("   ".non_blank(), None);
}
