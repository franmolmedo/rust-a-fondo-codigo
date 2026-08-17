fn main() {
    let prefix = String::from("ru");
    let starts_with_prefix = |candidate: &str| candidate.starts_with(&prefix);

    assert!(starts_with_prefix("rust"));
    assert_eq!(prefix, "ru");
}
