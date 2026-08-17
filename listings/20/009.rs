fn only_static(text: &'static str) -> usize {
    text.len()
}

fn main() {
    let accepts_any: fn(&str) -> usize = only_static;
    let local = String::from("local");
    assert_eq!(accepts_any(&local), 5);
}
