#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

trait IdGenerator {
    fn next_id(&mut self) -> u64;
}

struct Sequential(u64);

impl IdGenerator for Sequential {
    fn next_id(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

fn register(ids: &mut impl IdGenerator, name: &str) -> User {
    User { id: ids.next_id(), name: name.to_owned() }
}

fn main() {
    let mut ids = Sequential(40);
    assert_eq!(register(&mut ids, "Ada").id, 41);
    assert_eq!(register(&mut ids, "Grace").id, 42);
}
