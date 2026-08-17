#[derive(Debug, PartialEq)]
enum Position {
    Inside { x: i32, y: i32 },
    Outside { x: i32, y: i32 },
}

fn classify(x: i32, y: i32) -> Position {
    match (x, y) {
        (x @ 0..=100, y @ 0..=100) => Position::Inside { x, y },
        (x, y) => Position::Outside { x, y },
    }
}

fn main() {
    assert_eq!(classify(20, 40), Position::Inside { x: 20, y: 40 });
    assert_eq!(classify(-1, 40), Position::Outside { x: -1, y: 40 });
}
