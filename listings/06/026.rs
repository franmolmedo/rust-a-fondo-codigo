fn normalized_lines(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn main() {
    let lines = {
        let input = String::from(" UNO \n DOS ");
        normalized_lines(&input)
    };

    assert_eq!(lines, ["uno", "dos"]);
}
