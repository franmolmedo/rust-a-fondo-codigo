use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RetryLimit(NonZeroU32);

impl RetryLimit {
    fn new(value: u32) -> Option<Self> {
        NonZeroU32::new(value).map(Self)
    }

    fn get(self) -> u32 {
        self.0.get()
    }
}

fn main() {
    assert_eq!(RetryLimit::new(3).map(RetryLimit::get), Some(3));
    assert_eq!(RetryLimit::new(0), None);
}
