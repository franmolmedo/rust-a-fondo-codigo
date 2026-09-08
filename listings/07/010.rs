#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq)]
pub enum RectangleError {
    ZeroWidth,
    ZeroHeight,
}

impl Rectangle {
    pub fn try_new(width: u32, height: u32) -> Result<Self, RectangleError> {
        if width == 0 {
            return Err(RectangleError::ZeroWidth);
        }
        if height == 0 {
            return Err(RectangleError::ZeroHeight);
        }
        Ok(Self { width, height })
    }

    pub fn area(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
    }
}

fn main() {
    assert_eq!(Rectangle::try_new(0, 4), Err(RectangleError::ZeroWidth));
    assert_eq!(Rectangle::try_new(3, 0), Err(RectangleError::ZeroHeight));
    assert_eq!(Rectangle::try_new(3, 4).unwrap().area(), 12);

    let largest = Rectangle::try_new(u32::MAX, u32::MAX).unwrap();
    assert_eq!(largest.area(), u64::from(u32::MAX) * u64::from(u32::MAX));
}
