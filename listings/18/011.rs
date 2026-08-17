trait Catalog {
    fn names(&self) -> impl Iterator<Item = &str>;
}

struct VecCatalog(Vec<String>);

impl Catalog for VecCatalog {
    fn names(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(String::as_str)
    }
}

fn joined<C: Catalog>(catalog: &C) -> String {
    catalog.names().collect::<Vec<_>>().join(", ")
}

fn main() {
    let catalog = VecCatalog(vec![String::from("Rust"), String::from("Cargo")]);
    assert_eq!(joined(&catalog), "Rust, Cargo");
}
