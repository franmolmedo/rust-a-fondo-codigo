fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut text = String::from("hello world");
    let first = first_word(&text);

    text.clear();
    // error[E0502]: mutable access overlaps the shared view

    println!("{first}");
}
