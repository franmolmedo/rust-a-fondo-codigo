use std::collections::HashMap;

trait Repository: Default {
    fn save(&mut self, key: &str, value: &str);
    fn find(&self, key: &str) -> Option<&str>;
    fn delete(&mut self, key: &str) -> bool;
}

#[derive(Default)]
struct MemoryRepository(HashMap<String, String>);

impl Repository for MemoryRepository {
    fn save(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_owned(), value.to_owned());
    }

    fn find(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.0.remove(key).is_some()
    }
}

fn assert_repository_contract<R: Repository>() {
    let mut repository = R::default();
    repository.save("language", "Rust");
    assert_eq!(repository.find("language"), Some("Rust"));
    assert!(repository.delete("language"));
    assert!(!repository.delete("language"));
}

fn main() {
    assert_repository_contract::<MemoryRepository>();
}
