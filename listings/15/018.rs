struct Report;

impl Report {
    fn label(&self) -> &'static str { "inherente" }
}

trait Label {
    fn label(&self) -> &'static str;
}

impl Label for Report {
    fn label(&self) -> &'static str { "trait" }
}

fn main() {
    let report = Report;
    assert_eq!(report.label(), "inherente");
    assert_eq!(Label::label(&report), "trait");
    assert_eq!(<Report as Label>::label(&report), "trait");
}
