fn normalized_names(names: &[String]) -> Vec<String> {
    names
        .iter()
        .map(|name| name.trim().to_lowercase())
        .filter(|name| !name.is_empty())
        .collect()
}

fn main() {
    let names = vec![String::from(" Ada "), String::from("  ")];
    assert_eq!(normalized_names(&names), [String::from("ada")]);
    assert_eq!(names[0], " Ada ");
}
