fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() { first } else { second }
}

fn main() {
    let long = String::from("una cadena larga");
    let result;

    {
        let short = String::from("breve");
        result = longest(&long, &short);
    }

    println!("{result}");
    // error[E0597]: short does not live long enough
}
