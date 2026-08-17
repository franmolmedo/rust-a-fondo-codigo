#[derive(Debug, PartialEq, Eq)]
struct Versioned<T> {
    version: u64,
    value: T,
}

impl<T> Versioned<T> {
    fn map<U>(self, operation: impl FnOnce(T) -> U) -> Versioned<U> {
        Versioned {
            version: self.version,
            value: operation(self.value),
        }
    }
}

fn main() {
    let entity = Versioned { version: 7, value: String::from("Rust") };
    assert_eq!(entity.map(|value| value.len()), Versioned { version: 7, value: 4 });
}
