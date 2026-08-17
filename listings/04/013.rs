#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let first = Point { x: 3, y: 4 };
    let second = first;

    assert_eq!(first, second);
}
