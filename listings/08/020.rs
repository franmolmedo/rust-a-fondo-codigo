fn require_name(value: Option<String>) -> Result<String, &'static str> {
    let Some(name) = value else {
        return Err("nombre ausente");
    };

    if let Some(first) = name.chars().next() {
        if !first.is_alphabetic() {
            return Err("el nombre debe empezar por una letra");
        }
    } else {
        return Err("nombre vacío");
    }

    Ok(name)
}

fn main() {
    assert_eq!(require_name(Some(String::from("Ada"))), Ok(String::from("Ada")));
    assert_eq!(require_name(None), Err("nombre ausente"));
    assert_eq!(require_name(Some(String::new())), Err("nombre vacío"));
    assert_eq!(
        require_name(Some(String::from("1Ada"))),
        Err("el nombre debe empezar por una letra")
    );
    assert!(matches!(Some(3), Some(value) if value > 0));
}
