#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 2, y: 3 };
    let x = &mut point.x;
    let y = &mut point.y;

    *x *= 10;
    *y *= 10;

    assert_eq!((point.x, point.y), (20, 30));
}
