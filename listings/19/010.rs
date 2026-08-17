trait Renderer {
    fn render(&self) -> String;
}

struct TextView<'a>(&'a str);

impl Renderer for TextView<'_> {
    fn render(&self) -> String {
        self.0.to_owned()
    }
}

fn boxed_view<'a>(text: &'a str) -> Box<dyn Renderer + 'a> {
    Box::new(TextView(text))
}

fn main() {
    let text = String::from("vista");
    let renderer = boxed_view(&text);
    assert_eq!(renderer.render(), "vista");
}
