fn sum_pair((left, right): (i32, i32)) -> i32 {
    left + right
}

fn main() {
    let (x, y) = (10, 20);
    let add = |(left, right): (i32, i32)| left + right;
    let values = ["a", "b"];
    let indexed: Vec<_> = values.iter().enumerate().collect();

    assert_eq!((x, y), (10, 20));
    assert_eq!(sum_pair((2, 3)), 5);
    assert_eq!(add((4, 5)), 9);
    assert_eq!(indexed[1], (1, &"b"));
}
