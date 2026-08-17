fn append_mark(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("hola");
    let reference = &mut text;

    append_mark(reference);
    append_mark(reference);
    reference.make_ascii_uppercase();

    assert_eq!(text, "HOLA!!");
}
