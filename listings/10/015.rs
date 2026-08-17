#[derive(Debug)]
struct Percentage(u8);

impl Percentage {
    fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }

    fn complement(&self) -> u8 {
        debug_assert!(self.0 <= 100, "Percentage conserva su invariante");
        100 - self.0
    }
}

fn main() {
    let progress = Percentage::new(35).unwrap();
    assert_eq!(progress.complement(), 65);
    assert!(Percentage::new(101).is_none());
}
