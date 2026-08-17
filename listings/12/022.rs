fn default_names() -> impl Iterator<Item = String> {
    vec![String::from("Ada"), String::from("Grace")].into_iter()
}

fn main() {
    let names: Vec<_> = default_names().collect();
    assert_eq!(names, [String::from("Ada"), String::from("Grace")]);
}
