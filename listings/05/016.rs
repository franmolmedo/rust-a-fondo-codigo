fn contains_name(names: &[String], wanted: &str) -> bool {
    names.iter().any(|name| name == wanted)
}

fn main() {
    let vector = vec![String::from("Ada"), String::from("Grace")];
    let array = [String::from("Linus"), String::from("Margaret")];

    assert!(contains_name(&vector, "Grace"));
    assert!(contains_name(&array, "Linus"));
}
