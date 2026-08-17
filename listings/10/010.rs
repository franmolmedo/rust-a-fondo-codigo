use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Quantity(u32);

#[derive(Debug, PartialEq, Eq)]
enum QuantityError {
    Zero,
    AboveMaximum { maximum: u32, actual: u32 },
}

impl TryFrom<u32> for Quantity {
    type Error = QuantityError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const MAXIMUM: u32 = 100;
        match value {
            0 => Err(QuantityError::Zero),
            value if value > MAXIMUM => Err(QuantityError::AboveMaximum {
                maximum: MAXIMUM,
                actual: value,
            }),
            value => Ok(Self(value)),
        }
    }
}

fn main() {
    assert_eq!(Quantity::try_from(3), Ok(Quantity(3)));
    assert_eq!(Quantity::try_from(0), Err(QuantityError::Zero));
}
