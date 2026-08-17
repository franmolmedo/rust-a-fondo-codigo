use std::fmt;

struct Lines(Vec<String>);

impl fmt::Display for Lines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("\n"))
    }
}

impl From<Vec<String>> for Lines {
    fn from(lines: Vec<String>) -> Self {
        Self(lines)
    }
}

fn main() {
    let lines = Lines::from(vec![String::from("uno"), String::from("dos")]);
    assert_eq!(lines.to_string(), "uno\ndos");
}
