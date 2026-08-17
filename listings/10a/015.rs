use std::error::Error;

fn load_limit(input: &str) -> Result<u32, Box<dyn Error>> {
    let limit = input.parse::<u32>()?;
    Ok(limit)
}

fn run() -> Result<(), Box<dyn Error>> {
    assert_eq!(load_limit("25")?, 25);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()
}
