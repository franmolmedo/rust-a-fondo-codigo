fn initials(first: &str, last: &str) -> String {
    let first = first.chars().next().unwrap_or('?');
    let last = last.chars().next().unwrap_or('?');
    format!("{first}{last}")
}

fn main() {
    let name = String::from("Ada");
    let first_view = &name;
    let second_view = &name;

    assert_eq!(initials(first_view, second_view), "AA");
    assert_eq!(name, "Ada");
}
