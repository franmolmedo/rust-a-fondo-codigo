use std::error::Error;

fn load_limit(input: &str) -> Result<u32, Box<dyn Error>> {
    let limit = input.parse::<u32>()?;
    Ok(limit)
}

fn main() -> Result<(), Box<dyn Error>> {
    let limit = load_limit("25")?;
    assert_eq!(limit, 25);
    Ok(())
}
