trait Renderer {
    fn render(&self) -> String;
}

struct Text(String);
struct Number(i64);

impl Renderer for Text {
    fn render(&self) -> String {
        self.0.clone()
    }
}

impl Renderer for Number {
    fn render(&self) -> String {
        self.0.to_string()
    }
}

fn render_all(items: &[Box<dyn Renderer>]) -> Vec<String> {
    items.iter().map(|item| item.render()).collect()
}

fn main() {
    let items: Vec<Box<dyn Renderer>> = vec![
        Box::new(Text(String::from("Rust"))),
        Box::new(Number(2024)),
    ];
    assert_eq!(render_all(&items), ["Rust", "2024"]);
}
