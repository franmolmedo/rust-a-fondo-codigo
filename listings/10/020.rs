use std::error::Error;

fn read_answer(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(input.parse::<u32>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(read_answer("42")?, 42);
    Ok(())
}
