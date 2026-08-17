#[derive(Debug, PartialEq, Eq)]
struct Draft {
    body: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Published {
    body: String,
    revision: u64,
}

impl Draft {
    fn publish(self) -> Published {
        Published { body: self.body, revision: 1 }
    }
}

impl Published {
    fn body(&self) -> &str {
        &self.body
    }
}

fn main() {
    let published = Draft { body: String::from("Rust") }.publish();
    assert_eq!(published.body(), "Rust");
    assert_eq!(published.revision, 1);
}
