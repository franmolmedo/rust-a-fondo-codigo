fn main() {
    let text = String::from("hola");
    assert_eq!(text.len(), 4);
} // `text` se destruye aquí y libera su buffer
