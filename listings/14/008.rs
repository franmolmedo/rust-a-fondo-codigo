#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cents(u64);

#[derive(Debug, PartialEq, Eq)]
enum AmountError {
    Overflow,
    Insufficient { available: u64, requested: u64 },
}

impl Cents {
    fn checked_add(self, other: Self) -> Result<Self, AmountError> {
        self.0.checked_add(other.0).map(Self).ok_or(AmountError::Overflow)
    }

    fn checked_sub(self, other: Self) -> Result<Self, AmountError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(AmountError::Insufficient {
                available: self.0,
                requested: other.0,
            })
    }
}

fn main() {
    assert_eq!(Cents(500).checked_add(Cents(250)), Ok(Cents(750)));
    assert_eq!(
        Cents(300).checked_sub(Cents(500)),
        Err(AmountError::Insufficient { available: 300, requested: 500 })
    );
}
