fn main() {
    let summary = {
        let raw = String::from("  Ada Lovelace  ");
        let clean = raw.trim();
        format!("nombre: {clean}")
    }; // `raw` se destruye; `summary` conserva un String independiente

    assert_eq!(summary, "nombre: Ada Lovelace");
}
