fn main() {
    let maybe_text = Some(String::from("hola"));

    if let Some(_) = maybe_text {
        println!("hay texto");
    }

    assert_eq!(maybe_text.as_deref(), Some("hola"));
}
