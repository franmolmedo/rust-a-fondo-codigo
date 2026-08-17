trait Renderer {
    fn render(&self) -> String;
}

struct TextView<'a>(&'a str);

impl Renderer for TextView<'_> {
    fn render(&self) -> String {
        self.0.to_owned()
    }
}

fn boxed_view(text: &str) -> Box<dyn Renderer> {
    Box::new(TextView(text))
}

fn main() {}
