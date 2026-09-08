// build.rs
fn main() {
    // Vigilar este archivo como entrada del script:
    println!("cargo::rerun-if-changed=schema/catalog.proto");
    // Hacer que esta constante esté disponible al compilar la crate:
    println!("cargo::rustc-env=CATALOG_SCHEMA_VERSION=3");
}
