#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rectangles = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut calls = 0;

    rectangles.sort_by_key(|rectangle| {
        calls += 1;
        rectangle.width
    });

    assert_eq!(rectangles[0].width, 3);
    assert!(calls >= rectangles.len());
}
