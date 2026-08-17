fn validate_nonempty(text: &str) -> Result<(), &'static str> {
    if text.is_empty() {
        Err("texto vacío")
    } else {
        Ok(())
    }
}

fn main() {
    assert_eq!(validate_nonempty("rust"), Ok(()));
    assert!(validate_nonempty("").is_err());
}
