trait Left {
    type Value;
    fn value(&self) -> Self::Value;
}

trait Right {
    type Value;
    fn value(&self) -> Self::Value;
}

struct Both;

impl Left for Both {
    type Value = u32;
    fn value(&self) -> u32 { 10 }
}

impl Right for Both {
    type Value = &'static str;
    fn value(&self) -> &'static str { "ten" }
}

fn main() {
    let both = Both;
    let number: <Both as Left>::Value = <Both as Left>::value(&both);
    let text: <Both as Right>::Value = <Both as Right>::value(&both);
    assert_eq!(number, 10);
    assert_eq!(text, "ten");
}
