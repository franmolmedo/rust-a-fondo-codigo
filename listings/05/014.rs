fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

fn main() {
    let owned = String::from("   ");

    assert!(is_blank(&owned));
    assert!(is_blank(" \n "));
}
