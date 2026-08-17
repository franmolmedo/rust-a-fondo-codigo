use std::fmt::Debug;

fn length<'a>(text: &'a str) -> impl Copy + Debug + PartialEq<usize> + use<> {
    text.len()
}

fn main() {
    let answer;
    {
        let text = String::from("Rust");
        answer = length(&text);
    }
    assert_eq!(answer, 4);
}
