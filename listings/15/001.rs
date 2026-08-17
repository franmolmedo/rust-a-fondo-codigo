trait Summary {
    fn title(&self) -> &str;

    fn summary(&self) -> String {
        self.title().to_owned()
    }
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn title(&self) -> &str {
        &self.title
    }
}

fn render(item: &impl Summary) -> String {
    format!("Resumen: {}", item.summary())
}

fn main() {
    let article = Article { title: String::from("Ownership") };
    assert_eq!(render(&article), "Resumen: Ownership");
}
