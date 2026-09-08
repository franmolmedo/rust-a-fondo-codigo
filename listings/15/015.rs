#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

trait IdGenerator {
    fn next_id(&mut self) -> Option<u64>;
}

struct Sequential(u64);

impl IdGenerator for Sequential {
    fn next_id(&mut self) -> Option<u64> {
        self.0 = self.0.checked_add(1)?;
        Some(self.0)
    }
}

fn register(ids: &mut impl IdGenerator, name: &str) -> Option<User> {
    Some(User { id: ids.next_id()?, name: name.to_owned() })
}

fn main() {
    let mut ids = Sequential(40);
    assert_eq!(register(&mut ids, "Ada").unwrap().id, 41);
    assert_eq!(register(&mut ids, "Grace").unwrap().id, 42);
    let mut exhausted = Sequential(u64::MAX);
    assert_eq!(register(&mut exhausted, "Linus"), None);
    assert_eq!(exhausted.0, u64::MAX);
}
