#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn describe(point: Point) -> String {
    match point {
        Point { x: 0, y } => format!("y axis at {y}"),
        Point { x, y: 0 } => format!("x axis at {x}"),
        Point { x, y } => format!("{x}, {y}"),
    }
}

fn main() {
    let point = Point { x: 3, y: 4 };
    let Point {
        x: horizontal,
        y: vertical,
    } = point;

    assert_eq!((horizontal, vertical), (3, 4));
    assert_eq!(describe(Point { x: 0, y: 5 }), "y axis at 5");
}
