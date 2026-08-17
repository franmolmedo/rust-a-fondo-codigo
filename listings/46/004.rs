#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Percentage(u8);

impl Percentage {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

assert_eq!(std::mem::size_of::<Percentage>(), std::mem::size_of::<u8>());
assert_eq!(Percentage::new(75).map(Percentage::get), Some(75));
assert_eq!(Percentage::new(101), None);
