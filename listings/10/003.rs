fn normalized_tag(input: Option<&str>) -> Option<String> {
    input
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(str::to_lowercase)
}

fn label_length(label: &Option<String>) -> Option<usize> {
    label.as_deref().map(str::len)
}

fn main() {
    assert_eq!(normalized_tag(Some("  Rust ")), Some(String::from("rust")));
    assert_eq!(normalized_tag(Some("   ")), None);

    let label = Some(String::from("owned"));
    assert_eq!(label_length(&label), Some(5));
    assert_eq!(label.as_deref(), Some("owned"));
}
