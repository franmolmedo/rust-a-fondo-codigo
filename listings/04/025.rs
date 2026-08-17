fn main() {
    let text = String::from("hola");
    let attempts = 3_u32;

    let describe = move || format!("{text}: intento {attempts}");

    assert_eq!(attempts, 3);
    assert_eq!(describe(), "hola: intento 3");
    assert_eq!(describe(), "hola: intento 3");
}
