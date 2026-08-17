#[test]
fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original = Record::sample();
    let decoded = decode(&encode(&original)?)?;
    assert_eq!(decoded, original);
    Ok(())
}
