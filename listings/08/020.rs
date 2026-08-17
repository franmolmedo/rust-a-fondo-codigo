fn require_name(value: Option<String>) -> Result<String, &'static str> {
    let Some(name) = value else {
        return Err("nombre ausente");
    };

    if let Some(first) = name.chars().next() {
        assert!(first.is_alphabetic());
    }

    Ok(name)
}

fn main() {
    assert_eq!(require_name(Some(String::from("Ada"))), Ok(String::from("Ada")));
    assert_eq!(require_name(None), Err("nombre ausente"));
    assert!(matches!(Some(3), Some(value) if value > 0));
}
