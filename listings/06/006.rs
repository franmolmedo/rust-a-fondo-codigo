fn choose_first<'a, 'b>(first: &'a str, _second: &'b str) -> &'a str {
    first
}

fn main() {
    let persistent = String::from("permanezco");
    let result;

    {
        let temporary = String::from("temporal");
        result = choose_first(&persistent, &temporary);
    }

    assert_eq!(result, "permanezco");
}
