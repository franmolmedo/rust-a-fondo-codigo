#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
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
    rectangle.scale(2);
    assert_eq!(rectangle.into_dimensions(), (6, 8));
}
