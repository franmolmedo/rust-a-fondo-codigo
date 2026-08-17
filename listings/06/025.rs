fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn main() {
    let input = String::from(" uno \n\n dos ");
    let lines: Vec<_> = non_empty_lines(&input).collect();

    assert_eq!(lines, ["uno", "dos"]);
}
