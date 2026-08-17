fn main() {
    let name = String::from("Ada");
    let describe = || format!("Nombre: {name}");

    assert_eq!(describe(), "Nombre: Ada");
    assert_eq!(describe(), "Nombre: Ada");
    assert_eq!(name, "Ada");
}
