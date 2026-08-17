fn main() {
    let mut text = String::from("hola");

    let read = &text;
    assert_eq!(read.len(), 4);

    let write = &mut text;
    write.push('!');

    assert_eq!(text, "hola!");
}
