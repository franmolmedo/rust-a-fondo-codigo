trait Catalog {
    fn names(&self) -> impl Iterator<Item = &str>;
}

fn print_dynamic(_catalog: &dyn Catalog) {}

fn main() {}
