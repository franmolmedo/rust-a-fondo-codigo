fn selected_owned(names: &[String]) -> Vec<String> {
    names
        .iter()
        .filter(|name| name.starts_with('A'))
        .cloned()
        .collect()
}

fn main() {
    let names = vec![String::from("Ada"), String::from("Grace")];
    assert_eq!(selected_owned(&names), [String::from("Ada")]);
    assert_eq!(names.len(), 2);
}
