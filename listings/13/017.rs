fn normalized_words() -> impl Iterator<Item = String> {
    vec![String::from(" hola "), String::from(" mundo ")]
        .into_iter()
        .map(|value| value.trim().to_owned())
}

fn main() {
    assert_eq!(
        normalized_words().collect::<Vec<_>>(),
        [String::from("hola"), String::from("mundo")]
    );
}
