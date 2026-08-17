fn tuple_area(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    assert_eq!(tuple_area((3, 4)), 12);

    let rectangle = Rectangle {
        width: 3,
        height: 4,
    };
    assert_eq!(rectangle.area(), 12);
}
