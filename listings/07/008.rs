#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
    }

    fn scale(&mut self, factor: u32) -> Option<()> {
        let width = self.width.checked_mul(factor)?;
        let height = self.height.checked_mul(factor)?;
        self.width = width;
        self.height = height;
        Some(())
    }

    fn into_dimensions(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

fn main() {
    let mut rectangle = Rectangle {
        width: 3,
        height: 4,
    };

    assert_eq!(rectangle.area(), 12);
    assert_eq!(rectangle.scale(2), Some(()));
    assert_eq!(rectangle.into_dimensions(), (6, 8));

    let mut large = Rectangle {
        width: 1,
        height: u32::MAX,
    };
    assert_eq!(large.scale(2), None);
    assert_eq!(large.into_dimensions(), (1, u32::MAX));
}
