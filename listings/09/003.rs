fn sum_pair((left, right): (i32, i32)) -> i64 {
    i64::from(left) + i64::from(right)
}

fn main() {
    let (x, y) = (10, 20);
    let add = |(left, right): (i32, i32)| i64::from(left) + i64::from(right);
    let values = ["a", "b"];
    let indexed: Vec<_> = values.iter().enumerate().collect();

    assert_eq!((x, y), (10, 20));
    assert_eq!(sum_pair((2, 3)), 5);
    assert_eq!(add((4, 5)), 9);
    assert_eq!(sum_pair((i32::MAX, i32::MAX)), 4_294_967_294_i64);
    assert_eq!(add((i32::MIN, i32::MIN)), -4_294_967_296_i64);
    assert_eq!(indexed[1], (1, &"b"));
}
