fn require_value(value: Option<i32>) -> Result<i32, &'static str> {
    let Some(value) = value else {
        return Err("valor ausente");
    };
    Ok(value)
}

fn main() {
    assert_eq!(require_value(Some(10)), Ok(10));
    assert_eq!(require_value(None), Err("valor ausente"));
}
