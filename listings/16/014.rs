fn parse_pair<T>(left: &str, right: &str) -> Result<(T, T), T::Err>
where
    T: std::str::FromStr,
{
    Ok((left.parse()?, right.parse()?))
}

fn main() {
    let ports = parse_pair::<u16>("80", "443").unwrap();
    let coordinates: (i64, i64) = parse_pair("-3", "7").unwrap();
    assert_eq!(ports, (80, 443));
    assert_eq!(coordinates, (-3, 7));
}
