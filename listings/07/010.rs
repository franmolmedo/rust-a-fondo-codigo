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

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    assert_eq!(Rectangle::try_new(0, 4), Err(RectangleError::ZeroWidth));
    assert_eq!(Rectangle::try_new(3, 4).unwrap().area(), 12);
}
