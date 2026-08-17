fn assign<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let first = String::from("primero");
    let second = String::from("segundo");
    let mut slot: &str = &first;
    assign(&mut slot, &second);
    assert_eq!(slot, "segundo");
}
