fn main() {
    let summary = {
        let raw = String::from("  Ada Lovelace  ");
        let clean = raw.trim();
        format!("nombre: {clean}")
    }; // `clean` termina y `raw` se destruye; `summary` posee otro String

    assert_eq!(summary, "nombre: Ada Lovelace");
}
