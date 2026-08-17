trait Describe {
    fn describe(&self) -> String;
}

struct Text(String);
struct Count(usize);

impl Describe for Text {
    fn describe(&self) -> String {
        format!("texto de {} bytes", self.0.len())
    }
}

impl Describe for Count {
    fn describe(&self) -> String {
        format!("contador={}", self.0)
    }
}

fn main() {
    let values: Vec<Box<dyn Describe>> = vec![
        Box::new(Text(String::from("Rust"))),
        Box::new(Count(4)),
    ];
    let descriptions = values.iter().map(|value| value.describe()).collect::<Vec<_>>();
    assert_eq!(descriptions, ["texto de 4 bytes", "contador=4"]);
}
