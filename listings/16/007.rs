#[derive(Debug, PartialEq, Eq)]
struct Envelope<T> {
    value: T,
}

impl<T> Envelope<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn map<U>(self, operation: impl FnOnce(T) -> U) -> Envelope<U> {
        Envelope { value: operation(self.value) }
    }
}

impl<T: Clone> Envelope<T> {
    fn duplicate(&self) -> (T, T) {
        (self.value.clone(), self.value.clone())
    }
}

fn main() {
    let length = Envelope::new(String::from("Rust")).map(|text| text.len());
    assert_eq!(length, Envelope { value: 4 });
    assert_eq!(Envelope::new(7).duplicate(), (7, 7));
}
