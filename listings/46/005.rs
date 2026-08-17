#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum RecordTag {
    User = 1,
    Order = 2,
}

impl TryFrom<u8> for RecordTag {
    type Error = u8;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        match raw {
            1 => Ok(Self::User),
            2 => Ok(Self::Order),
            other => Err(other),
        }
    }
}

assert_eq!(RecordTag::try_from(2), Ok(RecordTag::Order));
assert_eq!(RecordTag::try_from(99), Err(99));
