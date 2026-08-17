pub struct Percentage(u8);

impl Percentage {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }
}
