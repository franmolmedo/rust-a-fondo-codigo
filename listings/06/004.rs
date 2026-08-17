fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let left = String::from("ferris");
    let right = String::from("rust");

    assert_eq!(longest(&left, &right), "ferris");
}
