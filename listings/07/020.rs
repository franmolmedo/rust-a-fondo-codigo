struct DraftPost {
    content: String,
}

struct PublishedPost {
    content: String,
}

impl DraftPost {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn publish(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut draft = DraftPost::new();
    draft.add_text("contenido");
    let published = draft.publish();

    assert_eq!(published.content(), "contenido");
}
