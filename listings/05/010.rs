#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 2, y: 3 };
    let x = &mut point.x;

    println!("{point:?}");
    // error[E0502]: the whole point cannot be read yet

    *x += 1;
}
