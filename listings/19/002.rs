trait Renderer {
    fn render(&self) -> String;
}

struct Label(&'static str);

impl Renderer for Label {
    fn render(&self) -> String {
        self.0.to_owned()
    }
}

fn borrowed(renderer: &dyn Renderer) -> String {
    renderer.render()
}

fn owned(renderer: Box<dyn Renderer>) -> String {
    renderer.render()
}

fn main() {
    let label = Label("prestado");
    assert_eq!(borrowed(&label), "prestado");

    let boxed: Box<dyn Renderer> = Box::new(Label("owned"));
    assert_eq!(owned(boxed), "owned");
}
