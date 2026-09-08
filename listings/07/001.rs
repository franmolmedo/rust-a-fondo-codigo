fn tuple_area(rectangle: (u32, u32)) -> u64 {
    u64::from(rectangle.0) * u64::from(rectangle.1)
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
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
