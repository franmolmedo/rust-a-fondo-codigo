fn main() {
    println!("cargo::rerun-if-changed=schema/catalog.proto");
    println!("cargo::rustc-env=CATALOG_SCHEMA_VERSION=3");
}
