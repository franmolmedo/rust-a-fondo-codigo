#[derive(Debug)]
struct Draft {
    title: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Published {
    title: String,
    revision: u64,
}

impl Draft {
    fn publish(self, revision: u64) -> Published {
        Published {
            title: self.title,
            revision,
        }
    }
}

fn main() {
    let draft = Draft { title: "Rust".into() };
    let published = draft.publish(1);
    assert_eq!(published, Published { title: "Rust".into(), revision: 1 });
}
