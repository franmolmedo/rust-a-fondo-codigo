trait Catalog {
    fn names(&self) -> Box<dyn Iterator<Item = &str> + '_>;
}

struct Words(Vec<String>);

impl Catalog for Words {
    fn names(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(self.0.iter().map(String::as_str))
    }
}

fn count(catalog: &dyn Catalog) -> usize {
    catalog.names().count()
}

fn main() {
    let words = Words(vec![String::from("uno"), String::from("dos")]);
    assert_eq!(count(&words), 2);
}
