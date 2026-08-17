fn pick(first: &str, second: &str) -> &str {
    if first.is_empty() { second } else { first }
    // error[E0106]: missing lifetime specifier
}

fn main() {}
