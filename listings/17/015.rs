use std::fmt::Display;

trait Source {
    type Item: Display;
    fn load(&self) -> Self::Item;
}

struct PortSource;

impl Source for PortSource {
    type Item = u16;
    fn load(&self) -> u16 { 8080 }
}

fn render_source(source: &impl Source) -> String {
    source.load().to_string()
}

fn main() {
    assert_eq!(render_source(&PortSource), "8080");
}
