// build.rs
fn main() {
    // Solo reejecutar si cambia el schema, no en cada build:
    println!("cargo::rerun-if-changed=schema/catalog.proto");
    // Exponer un valor calculado al código:
    println!("cargo::rustc-env=CATALOG_SCHEMA_VERSION=3");
}
