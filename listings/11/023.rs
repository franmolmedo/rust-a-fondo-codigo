fn prefix_filter(prefix: impl Into<String>) -> impl Fn(&str) -> bool {
    let prefix = prefix.into();
    move |candidate| candidate.starts_with(&prefix)
}

fn main() {
    let is_rust = prefix_filter("ru");
    assert!(is_rust("rust"));
    assert!(!is_rust("book"));
}
