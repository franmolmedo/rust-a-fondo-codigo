#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}

fn main() {
    assert_eq!(
        Rectangle::square(5),
        Rectangle {
            width: 5,
            height: 5,
        }
    );
}
