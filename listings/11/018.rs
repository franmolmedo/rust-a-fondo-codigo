fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |value| value + amount
}

fn main() {
    let add_ten = make_adder(10);
    assert_eq!(add_ten(5), 15);
    assert_eq!(add_ten(7), 17);
}
